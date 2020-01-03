use crate::matrix::Matrix;
use crate::vector::Vector;
use crate::point::Point;
use crate::ray::Ray;
use crate::canvas::{Canvas,Color};
use crate::world::World;

#[derive(PartialEq, Debug)]
pub struct Camera {
  pub hsize: u32,
  pub vsize: u32,
  pub fov: f64,
  pub transform: Matrix,
  pub pixel_size: f64,
  pub half_height: f64,
  pub half_width: f64,
  pub antialias: bool,
}

impl Camera {
  pub fn new(hsize: u32, vsize: u32, fov: f64) -> Camera {
    let half_view = (fov / 2.0).tan();
    let aspect = (hsize as f64 / vsize as f64) as f64;
    let half_width: f64;
    let half_height: f64;

    if aspect >= 1.0 {
      half_width = half_view;
      half_height = half_view / aspect;
    } else {
      half_width = half_view * aspect;
      half_height = half_view;
    }

    let pixel_size = (half_width * 2.0) / hsize as f64;

    Camera {
      hsize,
      vsize,
      fov,
      transform: Matrix::identity(),
      half_width,
      half_height,
      pixel_size,
      antialias: false
    }
  }

  pub fn ray_for_pixel(&self, px: u32, py: u32, ox: f64, oy: f64) -> Ray {
    let x_offset = (px as f64 + ox) * self.pixel_size;
    let y_offset = (py as f64 + oy) * self.pixel_size;

    let world_x = self.half_width - x_offset;
    let world_y = self.half_height - y_offset;

    let pixel = self.transform.inverse().unwrap() * Point { x: world_x, y: world_y, z: -1.0 };
    let origin = self.transform.inverse().unwrap() * Point { x: 0.0, y: 0.0, z: 0.0 };
    let direction = (pixel - origin).normalize();

    Ray { origin, direction }
  }

  pub fn render(&self, w: World) -> Canvas {
    let mut canvas = Canvas::new(self.hsize, self.vsize);

    for y in 0..self.vsize {
      for x in 0..self.hsize {
        let c: Color;
        if self.antialias {
          let mut color = Color { r: 0.0, g: 0.0, b: 0.0 };
          for i in 0..2 {
            for j in 0..2 {
              let r = self.ray_for_pixel(x, y, 0.25+(i as f64 * 0.5), 0.25+(j as f64 * 0.5));
              color = color + w.color_at(r);
            }
          }
          c = color * (1.0/4.0);
        } else {
          let r = self.ray_for_pixel(x, y, 0.5, 0.5);
          c = w.color_at(r);
        }


        canvas.set_pixel(x, y, c);
      }
    }

    canvas
  }

  pub fn view_transform(from: Point, to: Point, up: Vector) -> Matrix {
    let forward = (to - from).normalize();
    let upn = up.normalize();
    let left = forward.cross(&upn);
    let true_up = left.cross(&forward);

    let orientation = Matrix {
      data: [
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0]
      ]
    };

    orientation * Matrix::translate(-from.x, -from.y, -from.z)
  }
}


#[cfg(test)]
mod tests {
  use crate::camera::Camera;
  use crate::matrix::Matrix;
  use crate::vector::Vector;
  use crate::point::Point;
  use crate::world::World;
  use crate::canvas::Color;
  use crate::utils::equal;

  #[test]
  fn transformation_matrix_for_default_orientation() {
    let from = Point { x: 0.0, y: 0.0, z: 0.0 };
    let to = Point { x: 0.0, y: 0.0, z: -1.0 };
    let up = Vector { x: 0.0, y: 1.0, z: 0.0 };

    let t = Camera::view_transform(from, to, up);

    assert_eq!(t, Matrix::identity());
  }

  #[test]
  fn transformation_matrix_for_looking_in_positive_z() {
    let from = Point { x: 0.0, y: 0.0, z: 0.0 };
    let to = Point { x: 0.0, y: 0.0, z: 1.0 };
    let up = Vector { x: 0.0, y: 1.0, z: 0.0 };

    let t = Camera::view_transform(from, to, up);

    assert_eq!(t, Matrix::scale(-1.0, 1.0, -1.0));
  }

  #[test]
  fn view_transformation_moves_the_world() {
    let from = Point { x: 0.0, y: 0.0, z: 8.0 };
    let to = Point { x: 0.0, y: 0.0, z: 0.0 };
    let up = Vector { x: 0.0, y: 1.0, z: 0.0 };

    let t = Camera::view_transform(from, to, up);

    assert_eq!(t, Matrix::translate(0.0, 0.0, -8.0));
  }

  #[test]
  fn arbitrary_view_transformation() {
    let from = Point { x: 1.0, y: 3.0, z: 2.0 };
    let to = Point { x: 4.0, y: -2.0, z: 8.0 };
    let up = Vector { x: 1.0, y: 1.0, z: 0.0 };

    let t = Camera::view_transform(from, to, up);

    assert_eq!(t, Matrix {
      data: [
        [-0.50709,  0.50709,  0.67612, -2.36643],
        [ 0.76772,  0.60609,  0.12122, -2.82843],
        [-0.35857,  0.59761, -0.71714,  0.00000],
        [ 0.00000,  0.00000,  0.00000,  1.00000],
      ]
    });
  }

  #[test]
  fn constructing_a_camera() {
    let hsize = 160;
    let vsize = 120;
    let fov = std::f64::consts::PI / 2.0;

    let c = Camera::new(hsize, vsize, fov);

    assert_eq!(c.hsize, 160);
    assert_eq!(c.vsize, 120);
    assert_eq!(c.fov, std::f64::consts::PI / 2.0);
    assert_eq!(c.transform, Matrix::identity());
  }

  #[test]
  fn pixel_size_for_horizontal_canvas() {
    let hsize = 200;
    let vsize = 125;
    let fov = std::f64::consts::PI / 2.0;

    let c = Camera::new(hsize, vsize, fov);

    assert!(equal(c.pixel_size, 0.01));
  }

  #[test]
  fn pixel_size_for_vertical_canvas() {
    let hsize = 125;
    let vsize = 200;
    let fov = std::f64::consts::PI / 2.0;

    let c = Camera::new(hsize, vsize, fov);

    assert!(equal(c.pixel_size, 0.01));
  }

  #[test]
  fn ray_through_center_of_canvas() {
    let c = Camera::new(201, 101, std::f64::consts::PI / 2.0);
    let r = c.ray_for_pixel(100, 50, 0.5, 0.5);

    assert_eq!(r.origin, Point { x: 0.0, y: 0.0, z: 0.0 });
    assert_eq!(r.direction, Vector { x: 0.0, y: 0.0, z: -1.0 });
  }

  #[test]
  fn ray_through_corner_of_canvas() {
    let c = Camera::new(201, 101, std::f64::consts::PI / 2.0);
    let r = c.ray_for_pixel(0, 0, 0.5, 0.5);

    assert_eq!(r.origin, Point { x: 0.0, y: 0.0, z: 0.0 });
    assert_eq!(r.direction, Vector { x: 0.6651864, y: 0.33259323, z: -0.66851234 });
  }

  #[test]
  fn ray_when_camera_is_transformed() {
    let mut c = Camera::new(201, 101, std::f64::consts::PI / 2.0);
    c.transform = Matrix::rotate_y(std::f64::consts::PI / 4.0) * Matrix::translate(0.0, -2.0, 5.0);
    let r = c.ray_for_pixel(100, 50, 0.5, 0.5);

    assert_eq!(r.origin, Point { x: 0.0, y: 2.0, z: -5.0 });
    assert_eq!(r.direction, Vector { x: ((2.0 as f64).sqrt() / 2.0), y: 0.0, z: -((2.0 as f64).sqrt() / 2.0) });
  }

  #[test]
  fn render_world_with_camera() {
    let w = World::default();
    let mut c = Camera::new(11, 11, std::f64::consts::PI / 2.0);

    let from = Point { x: 0.0, y: 0.0, z: -5.0 };
    let to = Point { x: 0.0, y: 0.0, z: 0.0 };
    let up = Vector { x: 0.0, y: 1.0, z: 0.0 };
    c.transform = Camera::view_transform(from, to, up);

    let image = c.render(w);

    assert_eq!(image.get_pixel(5, 5), Color { r: 0.38039216, g: 0.4745098, b: 0.28235295 });
  }
}
