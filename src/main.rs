
use std::{self, thread};
use std::io::Write;
use rand::{Rng, FromEntropy};
use rand::rngs::SmallRng;

extern crate num_cpus;
// Naive port of http://fabiensanglard.net/postcard_pathtracer/ from C/C++ to Rust

#[derive(Debug, Copy, Clone)]
struct Vec3 {
    x: f32,
    y: f32,
    z: f32
}

impl Vec3 {
    pub fn dot(&self, other: &Vec3) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn normalized(&self) -> Vec3 {
        let invsqrt : f32 = self.dot(self).sqrt().recip();
        self * invsqrt 
    }
}

impl std::ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl std::ops::Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<'a> std::ops::Add<&Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl<'a> std::ops::Add<Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl std::ops::Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

impl<'a> std::ops::Add<f32> for &'a Vec3 {
    type Output = Vec3;

    fn add(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other
        }
    }
}

impl std::ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl std::ops::AddAssign<&Vec3> for Vec3 {
    fn add_assign(&mut self, other: &Vec3) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl<'a> std::ops::Sub<&Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z
        }
    }
}

impl<'a> std::ops::Mul<&Vec3> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z
        }
    }
}

impl std::ops::Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

impl<'a> std::ops::Mul<f32> for &'a Vec3 {
    type Output = Vec3;

    fn mul(self, other: f32) -> Vec3 {
        Vec3 {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other
        }
    }
}

fn box_test(pos: &Vec3, lo: &Vec3, hi: &Vec3) -> f32 {
    let lo = pos - lo;
    let hi = hi - pos;
    return -lo.x.min(hi.x).min(lo.y.min(hi.y)).min(lo.z.min(hi.z));
}

enum HitType {
    None,
    Letter,
    Wall,
    Sun
}

fn sample(pos: &Vec3) -> (f32, HitType) {
    let mut hit = HitType::Letter; // unless overridden by walls/sun below
    let mut dist : f32 = 1e9;

    // Letters
    {
        // Distance to letter segments in X-Y plane
        let origin = Vec3{ z: 0.0, ..*pos };
        let data : &'static[u8; 15*4] = b"5O5_5W9W5_9_AOEOCOC_A_E_IOQ_I_QOUOY_Y_]OWW[WaOa_aWeWa_e_cWiO";
        for i in 0..15 {
            let begin = Vec3{ x: data[i*4] as f32 - 79.0, y: data[i*4+1] as f32 - 79.0, z: 0.0 } * 0.5;
            let end = Vec3{ x: data[i*4+2] as f32 - 79.0, y: data[i*4+3] as f32 - 79.0, z: 0.0 } * 0.5;
            let delta = &end - &begin;
            let d = (&begin - &origin).dot(&delta) / delta.dot(&delta);
            let v = &origin - &(&begin + &delta * (-d.min(0.0)).min(1.0));
            dist = dist.min(v.dot(&v));
        }
        dist = dist.sqrt();

        // Curves for P & R, hard-coded, also in X-Y plane
        let curves : [Vec3; 2] = [
            Vec3{ x: -11.0, y: 6.0, z: 0.0 },
            Vec3{ x: 11.0, y: 6.0, z: 0.0 },
        ];
        for c in &curves {
            let mut o = &origin - c;
            let curve_dist : f32;
            if o.x > 0.0 {
                curve_dist = o.dot(&o).sqrt() - 2.0;
            }
            else {
                o.y += if o.y > 0.0 { -2.0 } else { 2.0 };
                curve_dist = o.dot(&o).sqrt(); 
            }
            dist = dist.min(curve_dist);
        }
        
        // Letter 3D-ifying and "rounding"
        dist = (dist.powf(8.0) + pos.z.powf(8.0)).powf(0.125) - 0.5;
    }
    
    // Room
    {
        let carved_box_room_dist = box_test(&pos, &Vec3{ x:-30.0, y:-0.5, z:-30.0 }, &Vec3{ x:30.0, y:18.0, z:30.0 });
        let carved_ceiling_dist = box_test(&pos, &Vec3{ x:-25.0, y:17.0, z:-25.0 }, &Vec3{ x:25.0, y:20.0, z:25.0 });
        let ceiling_beams_dist = box_test(
                &Vec3{
                    x: (pos.x.abs() / 8.0).fract() * 8.0, // |x| % 8
                    ..*pos
                },
                &Vec3{ x:1.5, y:18.5, z:-25.0},
                &Vec3{ x:6.5, y:20.0, z:25.0 }
            );
        let room_dist = (-carved_box_room_dist.min(carved_ceiling_dist)).min(ceiling_beams_dist);
        if room_dist < dist {
            dist = room_dist;
            hit = HitType::Wall;
        }
    }

    // Everything above 19.9 is sun
    {
        let sun_dist = 19.9 - pos.y;
        if sun_dist < dist {
            dist = sun_dist;
            hit = HitType::Sun;
        }
    }

    return (dist, hit);
}

struct Hit {
    pos : Vec3,
    norm : Vec3
}

fn ray_march(pos: &Vec3, dir: &Vec3, hit: &mut Hit) -> HitType {
    let mut no_hit_count = 0;
    let mut total_dist : f32 = 0.0;
    while total_dist < 100.0 {
        hit.pos = pos + dir * total_dist;
        let (dist, hit_type) = sample(&hit.pos);
        no_hit_count += 1;
        if (dist < 0.01) || (no_hit_count > 99) {
            let nx = sample(&(&hit.pos + Vec3{ x: 0.01, y: 0.0, z: 0.0 })).0 - dist;
            let ny = sample(&(&hit.pos + Vec3{ x: 0.0, y: 0.01, z: 0.0 })).0 - dist;
            let nz = sample(&(&hit.pos + Vec3{ x: 0.0, y: 0.0, z: 0.01 })).0 - dist;
            hit.norm = Vec3{ x: nx, y: ny, z: nz }.normalized();
            return hit_type;
        }
        total_dist += dist;
    }
    return HitType::None;
}

fn trace<R: Rng + ?Sized>(pos: &Vec3, dir: &Vec3, rng: &mut R) -> Vec3 {
    let mut origin = *pos;
    let mut direction = *dir;
    let mut hit = Hit{ pos: origin, norm: origin }; // useless init
    let mut color = Vec3{ x: 0.0, y: 0.0, z: 0.0 };
    let mut attenuation = Vec3{ x: 1.0, y: 1.0, z: 1.0 };
    let light_dir = Vec3{ x: 0.6, y: 0.6, z: 1.0 }.normalized();
    for _bounce in 0..3 {
        match ray_march(&origin, &direction, &mut hit) {
            HitType::None => break,
            HitType::Letter => {
                direction += &hit.norm * (-2.0 * hit.norm.dot(&direction));
                origin = &hit.pos + &direction * 0.1;
                attenuation = &attenuation * 0.2;
            },
            HitType::Wall => {
                let incidence = hit.norm.dot(&light_dir);
                let p = 6.283185 * rng.gen::<f32>();
                let c = rng.gen::<f32>();
                let s = (1.0 - c).sqrt();
                let g = if hit.norm.z < 0.0 { -1.0 } else { 1.0 };
                let u = -1.0 / (g + hit.norm.z);
                let v = hit.norm.x * hit.norm.y * u;
                direction = Vec3{
                    x: v,
                    y: g + hit.norm.y * hit.norm.y * u,
                    z: -hit.norm.y
                } * (p.cos() * s)
                +
                Vec3{
                    x: 1.0 + g * hit.norm.x * hit.norm.x * u,
                    y: g * v,
                    z: -g * hit.norm.x
                } * (p.sin() * s)
                +
                hit.norm * c.sqrt();
                origin = &hit.pos + &direction * 0.1;
                attenuation = &attenuation * 0.2;
                if incidence > 0.0 {
                    if let HitType::Sun = ray_march(&(&origin + &(&hit.norm * 0.1)), &light_dir, &mut hit) {
                        color += &attenuation * &Vec3{ x: 500.0, y: 400.0, z: 100.0 } * incidence;
                    }
                }

                //color = (&hit.norm * 15.0) + 15.0;
                //break;
            },
            HitType::Sun => {
                color += &attenuation * &Vec3{ x: 50.0, y: 80.0, z: 100.0 };
                break;
            }
        }
    }
    return color;
}

fn main() -> std::io::Result<()> {
    let width : i32 = 960;
    let height : i32 = 540;
    let samplecount = 16;

    let position = Vec3{ x:-22.0, y:5.0, z:25.0 };
    let goal = (&Vec3{ x:-3.0, y:4.0, z:0.0 } - &position).normalized();
    let left = Vec3{ x:goal.z, y:0.0, z:-goal.x }.normalized() * (1.0 / (width as f32));
    let up = Vec3{
        x: goal.y * left.z - goal.z * left.y,
        y: goal.z * left.x - goal.x * left.z,
        z: goal.x * left.y - goal.y * left.x
    };

    // Write the image header
    print!("P6 {} {} 255 ", width, height);
    
    let num_threads : i32 = num_cpus::get() as i32;
    let mut thread_handles = Vec::with_capacity(num_threads as usize);
    let height_block = (height + num_threads - 1) / num_threads;
    for n in 0..num_threads {
        let y_begin = n * height_block;
        let y_end = ((n + 1) * height_block).min(height);
        let handle = thread::spawn(move || {
            let mut rng = SmallRng::from_entropy();
            let data_size = ((y_end - y_begin) * width) as usize * 3;
            let mut data = Vec::with_capacity(data_size);
            let sample_norm = 1.0 / (samplecount as f32);
            let sample_bias = 14.0 / 241.0;
            for y in (y_begin..y_end).rev() {
                let fy0 : f32 = (y - height / 2) as f32;
                for x in (0..width).rev() {
                    let mut color = Vec3{ x:0.0, y:0.0, z:0.0 };
                    let fx0 : f32 = (x - width / 2) as f32;
                    for _ in 0..samplecount {
                        let fx : f32 = rng.gen();
                        let fy : f32 = rng.gen();
                        let dir = (goal + left * (fx0 + fx) + up * (fy0 + fy)).normalized();
                        color += trace(&position, &dir, &mut rng);
                    }
                    color = color * sample_norm + sample_bias;
                    let den = &color + 1.0;
                    color.x *= 255.0 / den.x;
                    color.y *= 255.0 / den.y;
                    color.z *= 255.0 / den.z;
                    data.push(color.x as u8);
                    data.push(color.y as u8);
                    data.push(color.z as u8);
                }
            }
            return data;
        });
        thread_handles.push(handle);
    }

    // Write data from threads in reverse order, last created thread first,
    // so that image rows are also written in reverse order.
    while !thread_handles.is_empty() {
        let handle = thread_handles.pop().unwrap();
        let data = handle.join().unwrap();
        std::io::stdout().write_all(&data[..])?;
    }

    Ok(())
}
