use raytracer::color::Color;
use parser;

#[test]
fn parsing_an_empty_file_results_in_eof_error() {
    let input = "";

    let output = parser::parse(input);
    let result = output.map_err(|err| err.to_string());

    assert_eq!(
        result,
        Err("EOF while parsing a value at line 1 column 0".to_string())
    );
}

#[test]
fn parsing_an_empty_object_is_an_invalid_scene() {
    let input = r#"{}"#;

    let output = parser::parse(input);
    let result = output.map_err(|err| err.to_string());

    assert_eq!(
        result,
        Err("missing field `camera` at line 1 column 2".to_string())
    );
}

#[test]
fn parsing_a_file_with_an_invalid_camera() {
    let input = r#"
{
    "camera": {
        "width": 1920,
        "height": 1080,
        "field_of_view": 60,
        "position": {
            "x": 0,
            "y": 0,
            "z": 0
        }
    },
    "world": {
        "objects": [],
        "lights": []
    }
}
    "#;

    let output = parser::parse(input);
    let result = output.map_err(|err| err.to_string());

    // TODO: Aqui podria mejorar el mensaje de error para mostrar cual es el valor por defecto de
    // `looking_to` si no se especifica.
    assert_eq!(
        result,
        Err(
            "`position` and `looking_to` points must be different: `{ x: 0, y: 0, z: 0 }`"
                .to_string()
        )
    );
}

#[test]
fn parsing_a_file_with_an_invalid_color_code() {
    let input = r#"
{
    "camera": {
        "width": 1920,
        "height": 1080,
        "field_of_view": 60,
        "position": {
            "x": 10,
            "y": 10,
            "z": 10
        }
    },
    "world": {
        "objects": [
            {
                "type": "sphere",
                "material": {
                    "texture": {
                        "type": "color",
                        "red": -10,
                        "green": 0,
                        "blue": 0
                    }
                }
            }
        ],
        "lights": []
    }
}
    "#;

    let output = parser::parse(input);
    let result = output.map_err(|err| err.to_string());

    assert_eq!(
        result,
        Err("invalid value: integer `-10`, expected u8 at line 26 column 9".to_string())
    )
}

#[test]
fn rendering_a_scene_from_a_file() {
    let input = r#"
{
    "camera": {
        "width": 11,
        "height": 11,
        "field_of_view": 90,
        "position": {
            "x": 0,
            "y": 0,
            "z": -5
        },
        "looking_to": {
            "x": 0,
            "y": 0,
            "z": 0
        }
    },
    "world": {
        "objects": [
            {
                "type": "sphere",
                "material": {
                    "diffuse": 0.7,
                    "specular": 0.2,
                    "texture": {
                        "type": "color",
                        "red": 204,
                        "green": 255,
                        "blue": 153
                    }
                }
            },
            {
                "type": "sphere",
                "transforms": [
                    {
                        "type": "scaling",
                        "x": 0.5,
                        "y": 0.5,
                        "z": 0.5
                    }
                ]
            }
        ],
        "lights": [
            {
                "position": {
                    "x": -10,
                    "y": 10,
                    "z": -10
                },
                "intensity": {
                    "red": 255,
                    "green": 255,
                    "blue": 255
                }
            }
        ]
    }
}
    "#;

    let scene = parser::parse(input).unwrap();
    let image = scene.render();

    assert_eq!(
        *image.pixel_at(5, 5),
        Color {
            red: 0.38066,
            green: 0.47583,
            blue: 0.2855
        }
    );
}
