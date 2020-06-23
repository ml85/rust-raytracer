use crate::color::Color;
use crate::lights::PointLight;
use crate::tuple::Tuple;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
}

impl Material {
    pub fn new() -> Self {
        Material {
            color: Color::new(1., 1., 1.),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }

    pub fn lighting(&self, light: PointLight, point: Tuple, eyev: Tuple, normalv: Tuple) -> Color {
        let effective_color = self.color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;

        let light_dot_normal = lightv.dot(normalv);
        let mut diffuse = Color::new(0., 0., 0.);
        let mut specular = Color::new(0., 0., 0.);
        if light_dot_normal >= 0. {
            diffuse = effective_color * self.diffuse * light_dot_normal;

            let reflectv = (-lightv).reflect(normalv);
            let reflect_dot_eye = reflectv.dot(eyev);

            if reflect_dot_eye > 0. {
                let factor = reflect_dot_eye.powf(self.shininess);
                specular = light.intensity * self.specular * factor;
            }
        }

        ambient + diffuse + specular
    }
}

#[cfg(test)]
mod tests {
    use crate::color::test_utils::assert_color_near;
    use crate::color::Color;
    use crate::lights::PointLight;
    use crate::materials::Material;
    use crate::tuple::{point, vector};

    #[test]
    fn defaut_material() {
        let m = Material::new();
        assert_eq!(Color::new(1., 1., 1.), m.color);
        assert_eq!(0.1, m.ambient);
        assert_eq!(0.9, m.diffuse);
        assert_eq!(0.9, m.specular);
        assert_eq!(200., m.shininess);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 0., -10.));
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(Color::new(1.9, 1.9, 1.9), result);
    }

    #[test]
    fn lighting_with_eye_between_light_and_surface_eye_offset_45_deg() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 2_f64.sqrt() / 2., 2_f64.sqrt() / 2.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 0., -10.));
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(Color::new(1.0, 1.0, 1.0), result);
    }

    #[test]
    fn lighting_with_eye_opposite_surface_light_offset_45_deg() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 10., -10.));
        let result = m.lighting(light, position, eyev, normalv);
        assert_color_near(Color::new(0.7364, 0.7364, 0.7364), result, 0.00001);
    }

    #[test]
    fn lighting_with_eye_in_path_of_the_reflection_vector() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., -2_f64.sqrt() / 2., -2_f64.sqrt() / 2.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 10., -10.));
        let result = m.lighting(light, position, eyev, normalv);
        assert_color_near(Color::new(1.6364, 1.6364, 1.6364), result, 0.00001);
    }

    #[test]
    fn lighting_light_behind_surface() {
        let m = Material::new();
        let position = point(0., 0., 0.);
        let eyev = vector(0., 0., -1.);
        let normalv = vector(0., 0., -1.);
        let light = PointLight::new(Color::new(1., 1., 1.), point(0., 0., 10.));
        let result = m.lighting(light, position, eyev, normalv);
        assert_eq!(Color::new(0.1, 0.1, 0.1), result);
    }
}