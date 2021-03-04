use rand::Rng;
use std::f32::consts;

fn main() {
    let net = Network::new(vec![2,1,2]);
    println!("{:?}", net);
    let result = feedforward(net.biases, net.weights, vec![0.1, 0.5]);
    println!("{:?}", result);
}

#[derive(Debug)]
struct Network {
    num_layer: u8,
    sizes: Vec<i16>,
    biases: Vec<Vec<f32>>,
    weights: Vec<Vec<Vec<f32>>>,
}

impl Network {
    fn new(sizes: Vec<i16>) -> Network {
        let num_layer: u8 = sizes.len() as u8;
        let sizes: Vec<i16> = sizes;
        let mut biases = Vec::new();
        let mut weights = Vec::new();
        let x = &sizes[..sizes.len()-1];
        let y = &sizes[1..];

        let mut rng = rand::thread_rng();

        // Preenchendo 'biases' com valores aleatórios
        // Lista de vetores com o tamanho de cada camada
        for i in 0..y.len() {
            let layer = (0..y[i]).map(|_| rng.gen_range::<f32>(0.0,1.0)).collect();
            biases.push(layer)
        }

        // Preenchendo weights
        // Lista com matrizes
        // weights[i][j][k] = peso do j elemento da camada i+1 em relação à saída k da camada i
        for i in 0..y.len() {
            let mut linha = Vec::new();
            for _ in 0..y[i] {
                let coluna: Vec<f32> = (0..x[i]).map(|_| rng.gen_range::<f32>(-1.0,1.0)).collect();
                linha.push(coluna);
            }
            weights.push(linha);
        }

        Network { num_layer, sizes, biases, weights }
    }
}

fn sigmoid(x: f32) -> f32 {
    1.0/(1.0 + f32::powf(consts::E, -x))
}

fn vec_sigmoid(v: Vec<f32>) -> Vec<f32> {
    let mut result: Vec<f32> = vec![0.0; v.len()];
    
    for i in 0..v.len() {
        result[i] = sigmoid(v[i])
    }

    result
}


fn feedforward(biases: Vec<Vec<f32>>, weights: Vec<Vec<Vec<f32>>>, a: Vec<f32>) -> Vec<f32> {
    let mut result = a;

    for i in 0..biases.len() {
        println!("Camada {}", i);
        let b = &biases[i];
        let w = &weights[i];
        let mut layer_output: Vec<f32> = Vec::new();
        layer_output.resize(w.len(), 0.0);

        for j in 0..w.len() {
            for k in 0..w[j].len() {
                layer_output[j] += w[j][k] * result[k];
            }
            layer_output[j] -= b[j];
        }

        println!("Camada finalizada");
        result = vec_sigmoid(layer_output);
    }

    result
}
