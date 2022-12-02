use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::UnorderedMap;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{log, near_bindgen, env, AccountId, Promise};

#[derive(Serialize, Deserialize, BorshSerialize, BorshDeserialize, Debug)]
#[serde(crate = "near_sdk::serde")]
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

#[cfg(not(target_arch = "wasm32"))]
#[cfg(test)]
mod tests {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::testing_env;

    use super::*;

    const ID: u64 = 0;
    const NOMBRE: &str = "JOHN DOE";
    const EDAD: &str = "100";
    const SEXO: &str = "M";


    fn set_context() {
        let mut context = VMContextBuilder::new();
        testing_env!(context.build());

        testing_env!(context
            .signer_account_id(ACCOUNT.parse().unwrap())
            .build());
    }

    #[test]
    pub fn test_set_paciente() {
        set_context();
        let mut contract = Contract::default();
        
        contract.set_paciente(ID, String::from(NAME), String::from(EDAD), String::from(SEXO));
        let c = contract.pacientes.get(&ID).unwrap();

        assert_eq!(c.id, ID);
        assert_eq!(c.account, ACCOUNT.to_string());
        assert_eq!(c.name, NAME);
        assert_eq!(c.description, DESCRIPTION);
    }
}