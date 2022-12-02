# chain_care contract

The smart contract exposes three methods to enable storing and retrieving patients information in the NEAR network.

```rust
const DEFAULT_GREETING: &str = "Hello";

pub struct Paciente {
    pub id: u64,
    pub cuenta_paciente: String,
    pub cuenta_doctor: String,
    pub nombre: String,
    pub sexo: String,
    pub origen: String,
    pub fecha_nacimiento: String,
    pub domicilio: String
    
}

impl Default for Paciente {
    fn default() -> Self {
        Paciente {
            id: 0,
            cuenta_paciente: String::from(""),
            cuenta_doctor: String::from(""),
            nombre: String::from(""),
            sexo: String::from(""),
            origen: String::from(""),
            fecha_nacimiento: String::from(""),
            domicilio: String::from("")
        }
    }
}




// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pacientes: UnorderedMap<u64, Paciente>,
    
}

// Define the default, which automatically initializes the contract
impl Default for Contract{
    fn default() -> Self{
        Self{
            pacientes: UnorderedMap::new(b"t".to_vec()),
            
        }
    }
}

impl Paciente {
    pub fn new(id: u64, cuenta_paciente: String, cuenta_doctor: String, nombre: String, sexo: String, origen: String, fecha_nacimiento: String, domicilio: String) -> Self {
        Self {
            id,
            cuenta_paciente,
            cuenta_doctor,
            nombre,
            sexo,
            origen,
            fecha_nacimiento,
            domicilio
        }
    }
}



// Implement the contract structure
#[near_bindgen]
impl Contract {
    pub fn get_paciente(&self, id: u64) -> Option<Paciente> {
        self.pacientes.get(&id)
    }

    
    pub fn set_paciente(&mut self, id:u64, cuenta_paciente: String, nombre: String, sexo: String, origen: String, fecha_nacimiento: String, domicilio: String) {
        
        let cuenta = env::signer_account_id().to_string();

        log!("Guardando paciente {}", nombre);
        let paciente = Paciente::new(id, String::from(&cuenta_paciente), cuenta.clone(), String::from(&nombre), String::from(&sexo), String::from(&origen), String::from(&fecha_nacimiento), String::from(&domicilio));
        self.pacientes.insert(&id, &paciente);
        let amount: u128 = 0_800_000_000_000_000_000_000_000; // 0.8 $NEAR as yoctoNEAR
        let account_id: AccountId = "hospitaltest.testnet".parse().unwrap();
        Promise::new(account_id).transfer(amount);
        

        env::log_str("Paciente creado correctamente");
    }

    pub fn get_pacientes(&self) -> Vec<Paciente>{
        self.pacientes.values_as_vector().to_vec()
    }

    
}
```

<br />

# Quickstart

1. Make sure you have installed [rust](https://rust.org/).
2. Install the [`NEAR CLI`](https://github.com/near/near-cli#setup)

<br />

## 1. Build and Deploy the Contract
You can automatically compile and deploy the contract in the NEAR testnet by running:

```bash
./build.sh
```

Once finished, deploy the contract:

```bash
near dev-deploy ./target/wasm32-unknown-unknown/release/historias_clinicas.wasm
```

<br />

## 2. Retrieve the patient information

`get_paciente` is a read-only method (`view` method).

`View` methods can be called for **free** by anyone, even people **without a NEAR account**!

```bash
# Use near-cli to get the information
near view <dev-account> get_paciente
```

<br />

## 3. Store a New Patient
`set_paciente` changes the contract's state, for which it is a `change` method.

`Change` methods can only be invoked using a NEAR account, since the account needs to pay GAS for the transaction.

```bash
# Use near-cli to set a new greeting
near call <dev-account> set_paciente '{"message":"howdy"}' --accountId <dev-account>
```

**Tip:** If you would like to call `set_paciente` using your own account, first login into NEAR using:

```bash
# Use near-cli to login your NEAR account
near login
```

and then use the logged account to sign the transaction: `--accountId <your-account>`.
