use raytracing::math::color::Color;
use raytracing::math::shape::{Plane, Shape, Sphere};
use raytracing::math::vec3d::Vec3D;
use raytracing::render::frame::Frame;
use raytracing::render::hittable::Hittable;
use raytracing::render::lightning::Light;
use raytracing::render::material::Material;
use raytracing::render::scene::Scene;
use std::fs::create_dir_all;
use std::path::Path;

fn main() {
    create_dir_all("images/").expect("Failed to create directory");
    render_box_scene("images/box_scene.png");
    render_tutorial_scene("images/tutorial_scene.png");
}

fn render_box_scene<Q: AsRef<Path>>(path: Q) {
    let glass = Material {
        specular: Color::unit() * 0.5,
        shininess: 125.0,
        reflectiveness: 0.1,
        transparency: 0.9,
        refractive_index: 1.5,
        ..Material::default()
    };
    let mirror = Material {
        specular: Color::unit() * 10.0,
        shininess: 1425.0,
        reflectiveness: 0.8,
        ..Material::default()
    };
    let basic_matt = Material {
        specular: Color::unit() * 0.05,
        shininess: 1.0,
        ..Material::default()
    };
    let basic_shiny = Material {
        specular: Color::unit() * 0.9,
        shininess: 100.0,
        reflectiveness: 0.2,
        ..Material::default()
    };

    let pale_yellow: Color<f64> = Color::new(253, 255, 194);
    let green_tea: Color<f64> = Color::new(201, 255, 194);
    let fresh_air: Color<f64> = Color::new(175, 228, 254);
    let vodka: Color<f64> = Color::new(196, 181, 255);

    let royal_purple: Color<f64> = Color::new(113, 78, 179);
    let united_nations_blue: Color<f64> = Color::new(82, 134, 218);
    let mantis: Color<f64> = Color::new(109, 187, 91);
    let minion_yellow: Color<f64> = Color::new(235, 224, 81);
    let royal_orange: Color<f64> = Color::new(249, 141, 82);
    let paradise_pink: Color<f64> = Color::new(225, 78, 101);

    let scene = Scene {
        background_color: Color::zero(),
        objects: vec![
            Hittable::Primitive {
                shape: Shape::Plane(Plane {
                    origin: Vec3D {
                        x: 0.0,
                        y: 20.0,
                        z: 0.0,
                    },
                    normal: Vec3D {
                        x: 0.0,
                        y: -1.0,
                        z: 0.0,
                    },
                }),
                material: Material {
                    diffuse: pale_yellow,
                    ..basic_matt
                },
            },
            Hittable::Primitive {
                shape: Shape::Plane(Plane {
                    origin: Vec3D {
                        x: 0.0,
                        y: -10.0,
                        z: 0.0,
                    },
                    normal: Vec3D {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                }),
                material: Material {
                    diffuse: vodka * 0.5,
                    ..basic_shiny
                },
            },
            Hittable::Primitive {
                shape: Shape::Plane(Plane {
                    origin: Vec3D {
                        x: 15.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    normal: Vec3D {
                        x: -1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                }),
                material: mirror,
            },
            Hittable::Primitive {
                shape: Shape::Plane(Plane {
                    origin: Vec3D {
                        x: -15.0,
                        y: 0.0,
                        z: 0.0,
                    },
                    normal: Vec3D {
                        x: 1.0,
                        y: 0.0,
                        z: 0.0,
                    },
                }),
                material: Material {
                    diffuse: green_tea * 0.9,
                    ..basic_matt
                },
            },
            Hittable::Primitive {
                shape: Shape::Plane(Plane {
                    origin: Vec3D {
                        x: 0.0,
                        y: 0.0,
                        z: 5.0,
                    },
                    normal: Vec3D {
                        x: 0.0,
                        y: 0.0,
                        z: -1.0,
                    },
                }),
                material: Material {
                    diffuse: fresh_air,
                    ..basic_matt
                },
            },
            Hittable::Primitive {
                shape: Shape::Plane(Plane {
                    origin: Vec3D {
                        x: 0.0,
                        y: 0.0,
                        z: -45.0,
                    },
                    normal: Vec3D {
                        x: 0.0,
                        y: 0.0,
                        z: 1.0,
                    },
                }),
                material: Material {
                    diffuse: fresh_air * 0.45,
                    specular: Color::zero(),
                    reflectiveness: 0.1,
                    ..Material::default()
                },
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: 9.0,
                        y: -6.5,
                        z: -30.0,
                    },
                    radius: 3.5,
                }),
                material: mirror,
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: 0.5,
                        y: -8.5,
                        z: -36.0,
                    },
                    radius: 1.5,
                }),
                material: glass,
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: 2.0,
                        y: -8.0,
                        z: -24.0,
                    },
                    radius: 2.0,
                }),
                material: Material {
                    diffuse: royal_purple * 0.7,
                    ..basic_shiny
                },
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: -3.0,
                        y: -8.0,
                        z: -28.0,
                    },
                    radius: 2.0,
                }),
                material: Material {
                    diffuse: royal_orange * 0.7,
                    ..basic_shiny
                },
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: -7.0,
                        y: -7.5,
                        z: -23.0,
                    },
                    radius: 2.5,
                }),
                material: Material {
                    diffuse: minion_yellow * 0.7,
                    ..basic_shiny
                },
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: -9.0,
                        y: -6.0,
                        z: -33.0,
                    },
                    radius: 4.0,
                }),
                material: Material {
                    diffuse: united_nations_blue * 0.7,
                    ..basic_shiny
                },
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: 12.0,
                        y: -8.8,
                        z: -26.0,
                    },
                    radius: 1.2,
                }),
                material: Material {
                    diffuse: mantis * 0.7,
                    ..basic_shiny
                },
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: -0.5,
                        y: -9.0,
                        z: -21.5,
                    },
                    radius: 1.0,
                }),
                material: Material {
                    ambient: paradise_pink * 0.3,
                    diffuse: paradise_pink * 0.6,
                    ..basic_matt
                },
            },
        ],
        lights: vec![
            Light {
                position: Vec3D {
                    x: 8.0,
                    y: 8.0,
                    z: 0.0,
                },
                intensity: 0.7,
            },
            Light {
                position: Vec3D {
                    x: -8.0,
                    y: 8.0,
                    z: 0.0,
                },
                intensity: 0.7,
            },
            Light {
                position: Vec3D {
                    x: 0.0,
                    y: 5.0,
                    z: -35.0,
                },
                intensity: 0.5,
            },
        ],
        recursion_depth: 6,
    };

    let mut frame: Frame<f64> = Frame::new(1024, 768, 60.0);
    frame.render(&scene);
    frame.save(path).expect("Failed to save image");
}

fn render_tutorial_scene<Q: AsRef<Path>>(path: Q) {
    let ivory = Material {
        diffuse: Color {
            r: 0.4,
            g: 0.4,
            b: 0.3,
        } * 0.6,
        specular: Color::unit() * 0.3,
        shininess: 50.0,
        reflectiveness: 0.1,
        ..Material::default()
    };
    let glass = Material {
        specular: Color::unit() * 0.5,
        shininess: 125.0,
        reflectiveness: 0.1,
        transparency: 0.8,
        refractive_index: 1.5,
        ..Material::default()
    };
    let red_rubber = Material {
        diffuse: Color {
            r: 0.4,
            g: 0.1,
            b: 0.1,
        } * 0.9,
        specular: Color::unit() * 0.1,
        shininess: 10.0,
        ..Material::default()
    };
    let mirror = Material {
        specular: Color::unit() * 10.0,
        shininess: 1425.0,
        reflectiveness: 0.8,
        ..Material::default()
    };

    let scene = Scene {
        background_color: Color {
            r: 0.2,
            g: 0.7,
            b: 0.8,
        },
        objects: vec![
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: -3.0,
                        y: 0.0,
                        z: -16.0,
                    },
                    radius: 2.0,
                }),
                material: ivory,
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: -1.0,
                        y: -1.5,
                        z: -12.0,
                    },
                    radius: 2.0,
                }),
                material: glass,
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: 1.5,
                        y: -0.5,
                        z: -18.0,
                    },
                    radius: 3.0,
                }),
                material: red_rubber,
            },
            Hittable::Primitive {
                shape: Shape::Sphere(Sphere {
                    center: Vec3D {
                        x: 7.0,
                        y: 5.0,
                        z: -18.0,
                    },
                    radius: 4.0,
                }),
                material: mirror,
            },
            Hittable::CheckerBoard {
                plane: Plane {
                    origin: Vec3D {
                        x: -10.0,
                        y: -4.0,
                        z: -30.0,
                    },
                    normal: Vec3D {
                        x: 0.0,
                        y: 1.0,
                        z: 0.0,
                    },
                },
                width: 20.0,
                basic_material: Material {
                    diffuse: Color::unit() * 0.3,
                    ..Material::default()
                },
                checker_color: Color {
                    r: 1.0,
                    g: 0.7,
                    b: 0.3,
                } * 0.3,
                checker_size: 2.0,
            },
        ],
        lights: vec![
            Light {
                position: Vec3D {
                    x: -20.0,
                    y: 20.0,
                    z: 20.0,
                },
                intensity: 1.5,
            },
            Light {
                position: Vec3D {
                    x: 30.0,
                    y: 50.0,
                    z: -25.0,
                },
                intensity: 1.8,
            },
            Light {
                position: Vec3D {
                    x: 30.0,
                    y: 20.0,
                    z: 30.0,
                },
                intensity: 1.7,
            },
        ],
        recursion_depth: 4,
    };

    let mut frame: Frame<f64> = Frame::new(1024, 768, 60.0);
    frame.render(&scene);
    frame.save(path).expect("Failed to save image");
}
