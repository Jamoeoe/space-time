pub fn length<const N: usize>(v: [f64; N]) -> f64 {
    let mut len = 0.0;
    for i in 0..N {
        len += v[i] * v[i];
    }
    len = len.sqrt();
    return len;
}

pub fn scale<const N: usize>(v: [f64; N], scalar: f64) -> [f64; N] {
    let mut scaled = [0.0f64; N];
    for i in 0..N {
        scaled[i] = v[i] * scalar;
    }
    return scaled;
}

pub fn normalize<const N: usize>(v: [f64; N]) -> [f64; N] {
    let length = length(v);
    return scale(v, 1.0 / length);
}

pub fn add<const N: usize>(v1: [f64; N], v2: [f64; N]) -> [f64; N] {
    let mut sum = [0.0f64; N];
    for i in 0..N {
        sum[i] = v1[i] + v2[i];
    }
    return sum;
}

pub fn subtract<const N: usize>(v1: [f64; N], v2: [f64; N]) -> [f64; N] {
    return add(v1, scale(v2, -1.0));
}

pub fn dot<const N: usize>(v1: [f64; N], v2: [f64; N]) -> f64 {
    let mut product = 0.0;
    for i in 0..N {
        product += v1[i] * v2[i];
    }
    return product;
}

//    let theta = (dot(v1, v2) / (length(v1) * length(v2))).acos();

pub fn cross3(v1: [f64; 3], v2: [f64; 3]) -> [f64; 3] {
    return [
        v1[1] * v2[2] - v1[2] * v2[1],
        v1[2] * v2[0] - v1[0] * v2[2],
        v1[0] * v2[1] - v1[1] * v2[0],
    ];
}

pub fn multiply_matrices<const N: usize, const M: usize, const P: usize>(
    m1: [[f64; M]; N],
    m2: [[f64; N]; P],
) -> [[f64; M]; P] {
    let mut product = [[0.0; M]; P];

    for i in 0..P {
        for j in 0..M {
            for k in 0..N {
                product[i][j] += m1[k][j] * m2[i][k];
            }
        }
    }

    return product;
}

// flip columns and rows
pub fn transpose_matrix<const N: usize, const M: usize>(m: [[f64; M]; N]) -> [[f64; N]; M] {
    let mut transposed = [[0.0; N]; M];
    for i in 0..M {
        for j in 0..N {
            transposed[i][j] = m[j][i];
        }
    }
    return transposed;
}

pub fn equals<const N: usize, const M: usize>(m1: [[f64; M]; N], m2: [[f64; M]; N]) -> bool {
    for i in 0..N {
        for j in 0..M {
            if (m1[i][j] - m2[i][j]).abs() > 0.00000001 {
                return false;
            }
        }
    }
    return true;
}

pub fn unit_vector_between_vectors<const N: usize>(v1: [f64; N], v2: [f64; N]) -> [f64; N] {
    let offset = subtract(v1, v2);
    return normalize(offset);
}

pub fn convert_f64_matrix_to_f32<const N: usize, const M: usize>(
    m1: [[f64; M]; N],
) -> [[f32; M]; N] {
    let mut converted = [[0f32; M]; N];
    for i in 0..N {
        for j in 0..M {
            converted[i][j] = m1[i][j] as f32;
        }
    }
    return converted;
}

pub fn convert_f64_matrix_to_f32_4x4(m1: [[f64; 4]; 4]) -> [[f32; 4]; 4] {
    let mut converted = [[0f32; 4]; 4];
    for i in 0..4 {
        for j in 0..4 {
            converted[i][j] = m1[i][j] as f32;
        }
    }
    return converted;
}
