/*
 * Blue Engine copyright 2021 © Elham Aryanpur
 *
 * The license is same as the one on the root.
*/

use image::{EncodableLayout, GenericImageView};

use crate::{
    header::{percentage, Renderer, TextureData},
    objects,
};
use std::{collections::BTreeMap, io::Write};

#[derive(Debug, Clone, Copy)]
struct TextData {
    content: &'static str,
    position: (f32, f32),
    color: &'static [f32; 4],
    scale: f32,
}
pub struct Text {
    font: fontdue::Font,
    char_cache: BTreeMap<char, (fontdue::Metrics, usize)>,
    size: f32,
}

impl Text {
    pub fn new(font: &[u8], cache_on_size: f32, renderer: &mut Renderer) -> anyhow::Result<Self> {
        let font = fontdue::Font::from_bytes(font, fontdue::FontSettings::default()).unwrap();
        let mut char_cache = BTreeMap::<char, (fontdue::Metrics, usize)>::new();

        let characters = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!@#$%^&*()_+-=/\\?|<>'\"{}[],.~`";
        for i in characters.chars() {
            let character = font.rasterize(i, cache_on_size);

            let mut char_image =
                image::RgbaImage::new(character.0.width as u32, character.0.height as u32);

            let mut char_length: usize = 0;
            for pixel in char_image.pixels_mut() {
                //let pixel_value = percentage(character.1[char_length] as f32, 255f32);]
                let pixel_value = character.1[char_length];
                pixel.0 = [pixel_value, pixel_value, pixel_value, 255];

                char_length += 1;
            }

            let index = renderer.build_and_append_texture(
                "Character Cache",
                TextureData::Image(image::DynamicImage::ImageRgba8(char_image)),
                crate::header::TextureMode::Clamp,
                //crate::header::TextureFormat::PNM,
            )?;
            char_cache.insert(i, (character.0, index)); // slap these as bmp textures
        }

        Ok(Self {
            font,
            char_cache,
            size: cache_on_size,
        })
    }

    pub fn draw(
        &mut self,
        content: &str,
        position: (usize, usize),
        engine: &mut crate::header::Engine,
    ) -> anyhow::Result<()> {
        //let mut chars = Vec::<Vertex>::new();
        for i in content.char_indices() {
            let character: (fontdue::Metrics, usize);
            match self.char_cache.get(&i.1) {
                Some(char) => character = char.clone(),
                None => {
                    character = {
                        let character = self.font.rasterize(i.1, self.size);
                        let mut char_image = image::RgbaImage::new(
                            character.0.width as u32,
                            character.0.height as u32,
                        );

                        let mut char_length: usize = 0;
                        for pixel in char_image.pixels_mut() {
                            //let pixel_value = percentage(character.1[char_length] as f32, 255f32);]
                            let pixel_value = character.1[char_length];
                            pixel.0 = [pixel_value, pixel_value, pixel_value, 255];

                            char_length += 1;
                        }
                        let index = engine.renderer.build_and_append_texture(
                            "charTest",
                            TextureData::Image(image::DynamicImage::ImageRgba8(char_image)),
                            crate::header::TextureMode::Clamp,
                            //crate::header::TextureFormat::BMP,
                        )?;
                        (character.0, index)
                    }
                }
            }

            let window_size = engine.window.inner_size();
            let character_shape_index = objects::square(Some("text"), engine)?;
            let character_shape = engine.get_object(character_shape_index)?;
            character_shape.resize(
                character.0.width as f32,
                character.0.height as f32,
                0.0,
                window_size,
            );
            character_shape.change_texture(character.1)?;
            character_shape.position((position.0 * i.0) as f32, position.1 as f32, 0.0, window_size);
        }
        Ok(())
    }
}
