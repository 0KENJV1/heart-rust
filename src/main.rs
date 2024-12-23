extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::window::Window;
use kiss3d::nalgebra::{Translation3, Vector3};
use std::f32::consts::PI;

fn heart_shape(u: f32, v: f32) -> Vector3<f32> {
    let x = 16.0 * (u).sin().powi(3);
    let y = 13.0 * (u).cos() - 5.0 * (2.0 * u).cos() - 2.0 * (3.0 * u).cos() - (4.0 * u).cos();
    let z = 2.5 * (v).cos(); // Se utiliza v para ajustar el "grosor" del corazón en z
    Vector3::new(x, y, z)
}

fn main() {
    // Crear la ventana de visualización
    let mut window = Window::new("Heart in 3D");

    // Variables para controlar la rotación
    let mut rotation_angle = 0.0;

    // Parametrización para la figura del corazón
    let resolution = 100; // Resolución para los puntos del corazón
    let radius = 1.0; // Tamaño del corazón
    let mut vertices = Vec::new();
    
    // Generar los vértices del corazón con la ecuación paramétrica
    for i in 0..resolution {
        let u = i as f32 * 2.0 * PI / resolution as f32;
        for j in 0..resolution {
            let v = j as f32 * PI / resolution as f32;
            let point = heart_shape(u, v);
            vertices.push(point * radius);
        }
    }

    // Crear esferas para representar el corazón como puntos
    let mut spheres = Vec::new();
    for point in vertices {
        let mut sphere = window.add_sphere(0.05); // Esferas pequeñas para representar los puntos
        sphere.set_local_translation(Translation3::new(point.x, point.y, point.z));
        sphere.set_color(1.0, 0.0, 0.0);
        spheres.push(sphere);
    }

    // Bucle de renderizado
    while window.render() {
        // Rotar el corazón para ver en diferentes ángulos
        rotation_angle += 0.01;
        if rotation_angle > 2.0 * PI {
            rotation_angle -= 2.0 * PI;
        }
        
        // Aplicar rotación a las esferas
        for sphere in &mut spheres {
            sphere.set_local_rotation(na::UnitQuaternion::from_euler_angles(0.0, rotation_angle, 0.0));
        }
    }
}



