use parser;

fn main() {
    let input = r#"
{
    "camera": {
        "width": 1280,
        "height": 720,
        "field_of_view": 60,
        "position": {
            "x": 10,
            "y": 3,
            "z": -10
        }
    },
    "world": {
        "objects": [
            {
                "type": "sphere",
                "material": {
                    "reflective": 0.25,
                    "texture": {
                        "type": "color",
                        "red": 127,
                        "green": 127,
                        "blue": 127
                    }
                },
                "transforms": [
                    {
                        "type": "translation",
                        "x": 4,
                        "y": 1,
                        "z": -4
                    }
                ]
            },
            {
                "type": "sphere",
                "material": {
                    "texture": {
                        "type": "color",
                        "red": 255,
                        "green": 0,
                        "blue": 0
                    }
                },
                "transforms": [
                    {
                        "type": "scaling",
                        "x": 0.5,
                        "y": 0.5,
                        "z": 0.5
                    },
                    {
                        "type": "translation",
                        "x": 4,
                        "y": 0.5,
                        "z": -6
                    }
                ]
            },
            {
                "type": "sphere",
                "material": {
                    "texture": {
                        "type": "color",
                        "red": 127,
                        "green": 127,
                        "blue": 230
                    }
                },
                "transforms": [
                    {
                        "type": "scaling",
                        "x": 0.25,
                        "y": 0.25,
                        "z": 0.25
                    },
                    {
                        "type": "translation",
                        "x": 6,
                        "y": 0.25,
                        "z": -4.5
                    }
                ]
            },
            {
                "type": "plane"
            },
            {
                "type": "plane",
                "material": {
                    "texture": {
                        "type": "pattern",
                        "pattern": "checker",
                        "from": {
                            "red": 255,
                            "green": 255,
                            "blue": 255
                        },
                        "to": {
                            "red": 0,
                            "green": 0,
                            "blue": 0
                        }
                    }
                },
                "transforms": [
                    {
                        "type": "rotation_z",
                        "degrees": 90
                    },
                    {
                        "type": "translation",
                        "x": 0,
                        "y": 0,
                        "z": 0
                    }
                ]
            },
            {
                "type": "plane",
                "material": {
                    "texture": {
                        "type": "pattern",
                        "pattern": "checker",
                        "from": {
                            "red": 255,
                            "green": 255,
                            "blue": 255
                        },
                        "to": {
                            "red": 0,
                            "green": 0,
                            "blue": 0
                        }
                    }
                },
                "transforms": [
                    {
                        "type": "rotation_x",
                        "degrees": 90
                    }
                ]
            }
        ],
        "lights": [
            {
                "position": {
                    "x": 5,
                    "y": 5,
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

    let image = parser::parse(input).unwrap();
    let image = image.render();
    println!("{}", image.to_ppm());
}
