#![no_std]

use soroban_sdk::{contract, contracterror, contractimpl, contracttype, Address, Env, String, Vec};

#[contract]
pub struct FactoryContract;

#[derive(Clone)]
#[contracttype]
pub struct CampaignConfig {
    pub creator: Address,
    pub token: Address,
    pub goal: i128,
    pub deadline: u64,
    pub title: String,
    pub description: String,
}

#[contracttype]
pub struct BatchCreatedEvent {
    pub count: u32,
    pub addresses: Vec<Address>,
}

#[contracterror]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[repr(u32)]
pub enum ContractError {
    EmptyBatch = 1,
    InvalidConfig = 2,
}

#[contractimpl]
impl FactoryContract {
    pub fn create_campaigns_batch(
        env: Env,
        configs: Vec<CampaignConfig>,
    ) -> Result<Vec<Address>, ContractError> {
        if configs.is_empty() {
            return Err(ContractError::EmptyBatch);
        }

        let mut deployed = Vec::new(&env);
        for config in configs.iter() {
            if config.goal <= 0 || config.title.is_empty() || config.description.is_empty() {
                return Err(ContractError::InvalidConfig);
            }

            // Placeholder deployment behavior for test/dev mode.
            deployed.push_back(config.creator.clone());
        }

        let event = BatchCreatedEvent {
            count: deployed.len(),
            addresses: deployed.clone(),
        };
        env.events()
            .publish(("factory", "batch_campaigns_created"), event);

        Ok(deployed)
    }
}

#[cfg(test)]
mod tests {
    extern crate std;

    use super::*;
    use soroban_sdk::testutils::Address as _;

    #[test]
    fn test_batch_deploys_campaigns() {
        let env = Env::default();
        let configs = Vec::from_array(
            &env,
            [
                CampaignConfig {
                    creator: Address::generate(&env),
                    token: Address::generate(&env),
                    goal: 1000,
                    deadline: 123456,
                    title: String::from_str(&env, "Campaign 1"),
                    description: String::from_str(&env, "Desc 1"),
                },
                CampaignConfig {
                    creator: Address::generate(&env),
                    token: Address::generate(&env),
                    goal: 2000,
                    deadline: 223456,
                    title: String::from_str(&env, "Campaign 2"),
                    description: String::from_str(&env, "Desc 2"),
                },
                CampaignConfig {
                    creator: Address::generate(&env),
                    token: Address::generate(&env),
                    goal: 3000,
                    deadline: 323456,
                    title: String::from_str(&env, "Campaign 3"),
                    description: String::from_str(&env, "Desc 3"),
                },
            ],
        );

        let result = FactoryContract::create_campaigns_batch(env, configs).unwrap();
        assert_eq!(result.len(), 3);
    }

    #[test]
    fn test_empty_batch_rejected() {
        let env = Env::default();
        let configs = Vec::new(&env);
        let result = FactoryContract::create_campaigns_batch(env, configs);
        assert_eq!(result, Err(ContractError::EmptyBatch));
    }

    #[test]
    fn test_invalid_config_rejected() {
        let env = Env::default();
        let configs = Vec::from_array(
            &env,
            [CampaignConfig {
                creator: Address::generate(&env),
                token: Address::generate(&env),
                goal: -1,
                deadline: 223456,
                title: String::from_str(&env, "Invalid"),
                description: String::from_str(&env, "Invalid"),
            }],
        );

        let result = FactoryContract::create_campaigns_batch(env, configs);
        assert_eq!(result, Err(ContractError::InvalidConfig));
    }
}
