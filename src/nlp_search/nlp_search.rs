use std::collections::HashMap;

type DocId = u16;

struct NPIndex {
    index: HashMap<String, HashMap<DocId, usize>>,
}

impl NPIndex {
    fn new() -> Self {
        Self {
            index: HashMap::new(),
        }
    }

    fn add_document(&mut self, doc_id: DocId, text: &str) {
        let candidates = extract_np_candidates(text);

        for np in candidates {
            let phrases = expand_np(&np);

            for phrase in phrases {
                let entry = self.index.entry(phrase).or_insert_with(HashMap::new);

                *entry.entry(doc_id).or_insert(0) += 1;
            }
        }
    }

    fn search(&self, query: &str) -> Vec<(DocId, usize)> {
        let mut scores: HashMap<DocId, usize> = HashMap::new();

        let candidates = extract_np_candidates(query);

        for np in candidates {
            let phrases = expand_np(&np);

            for phrase in phrases {
                if let Some(docs) = self.index.get(&phrase) {
                    for (doc, weight) in docs {
                        *scores.entry(*doc).or_insert(0) += *weight;
                    }
                }
            }
        }

        let mut results: Vec<(DocId, usize)> = scores.into_iter().collect();

        results.sort_by(|a, b| b.1.cmp(&a.1));

        results
    }
}

fn normalize(text: &str) -> String {
    text.to_lowercase()
        .replace(",", "")
        .replace(".", "")
        .replace(";", "")
}

fn tokenize(text: &str) -> Vec<String> {
    normalize(text)
        .split_whitespace()
        .map(|s| s.to_string())
        .collect()
}

fn extract_np_candidates(text: &str) -> Vec<String> {
    let tokens = tokenize(text);
    let mut phrases = Vec::new();

    for i in 0..tokens.len() {
        for j in i + 2..=(i + 10).min(tokens.len()) {
            let phrase = tokens[i..j].join(" ");
            phrases.push(phrase);
        }
    }

    phrases
}

fn expand_np(np: &str) -> Vec<String> {
    let words: Vec<&str> = np.split_whitespace().collect();
    let mut phrases = Vec::new();

    if words.len() < 2 {
        return phrases;
    }

    for i in 0..words.len() {
        for j in i + 2..=words.len() {
            phrases.push(words[i..j].join(" "));
        }
    }

    phrases
}

pub fn nlp_search_fn() {
    let mut index = NPIndex::new();

    let documents = vec![
         (
            1,
            "The financial market regulator announced new policies to control inflationary pressure
            on emerging economies. The regulator emphasized that economic pressure from global trade 
            conflicts has increased significantly. Financial analysts expect additional regulatory 
            measures to stabilize market conditions and reduce investor uncertainty."
        ),
        (
            2,
            "The hydraulic pressure regulator valve controls the output pressure of the main hydraulic line.
            The regulator valve ensures that the downstream pressure does not exceed the configured safety 
            threshold. In industrial hydraulic systems, pressure regulators are critical components that 
            protect pumps, pipes, and connected actuators from excessive pressure buildup."
        ),

        (
            3,
            "A hydraulic pressure regulator is responsible for stabilizing pressure fluctuations in a
            hydraulic circuit. The regulator monitors the upstream pressure and dynamically adjusts the 
            valve opening in order to maintain a constant output pressure. Modern pressure regulator 
            assemblies include pressure sensors, feedback controllers, and safety bypass valves."
        ),

        (
            4,
            "Output pressure sensors monitor the hydraulic system continuously and provide real time
            measurements to the control unit. These sensors are typically installed downstream of the 
            pressure regulator valve in order to detect anomalies such as pressure spikes, regulator 
            failure, or unexpected flow restrictions."
        ),

        (
            5,
            "The thermal management subsystem of the hydraulic unit includes cooling radiators,
            temperature sensors, and flow regulators. Excessive temperature can reduce the viscosity 
            of the hydraulic fluid and negatively impact the pressure regulator performance. Proper 
            temperature monitoring ensures stable hydraulic pressure and reliable actuator operation."
        ),

        (
            6,
            "In aerospace hydraulic systems, pressure regulator valves must operate under extreme
            conditions including rapid pressure changes, vibration, and temperature variations. 
            Engineers design redundant regulator mechanisms in order to guarantee that the hydraulic 
            pressure remains within operational limits during flight."
        ),

        (
            7,
            "The industrial automation platform uses multiple hydraulic subsystems to drive mechanical
            actuators. Each subsystem includes a pressure regulator, flow control valve, and pressure 
            relief valve. The regulator maintains a stable hydraulic pressure level so that actuator 
            movements remain precise and predictable."
        ),

        (
            8,
            "During maintenance procedures, technicians inspect the hydraulic pressure regulator
            assembly for contamination, worn seals, and valve misalignment. Any degradation in the 
            pressure regulation mechanism can cause unstable output pressure and lead to damage in 
            sensitive hydraulic components."
        ),

        (
            9,
            "Advanced hydraulic control systems integrate electronic controllers with mechanical
            pressure regulators. The electronic control unit receives measurements from pressure 
            sensors and adjusts the regulator valve position to maintain optimal pressure conditions 
            throughout the hydraulic circuit."
        ),

        (
            10,
            "The pressure distribution manifold routes hydraulic fluid to multiple actuator lines.
            Each branch of the manifold may contain its own pressure regulator and pressure monitoring 
            sensor in order to maintain stable operating conditions across the entire hydraulic network."
        ),

        (
            11,
            "Hydraulic test benches are used to validate the performance of pressure regulators under
            controlled conditions. Engineers simulate different load scenarios, pressure levels, and 
            flow rates to evaluate how the regulator valve reacts to sudden pressure changes."
        ),    
        ];
        for (id, text) in documents {
            index.add_document(id, text);
        }

        println!("Index created with {} phrases", index.index.len());

        let query = "hydraulic pressure regulator";

        println!("Query: {}\n", query);

        let results = index.search(query);

        println!("Results:");

        for (doc, score) in results {
            println!("Document {} | score {}", doc, score);
        }


}
