use rand::Rng;

fn main() {
    let net = Network::new(vec![1,2,3]);
    println!("{:?}", net)
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

        let mut rng = rand::thread_rng();

        for i in 0..sizes.len() {
            let layer = (0..sizes[i]).map(|_| rng.gen_range::<f32>(0.0,1.0)).collect();
            biases.push(layer)
        }

        let weights = vec![vec![vec![0.0]]];

        Network { num_layer, sizes, biases, weights }
    }
}


