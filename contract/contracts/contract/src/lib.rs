#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Review {
    pub reviewer: Address,
    pub score: u32,
}

#[contracttype]
pub enum DataKey {
    Reviews(Address),
}

#[contract]
pub struct TenantScoringContract;

#[contractimpl]
impl TenantScoringContract {

    pub fn submit_review(env: Env, tenant: Address, reviewer: Address, score: u32) {
        reviewer.require_auth();

        if score == 0 || score > 10 {
            panic!("Invalid score");
        }

        let key = DataKey::Reviews(tenant.clone());

        let mut reviews: Vec<Review> = env
            .storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env));

        reviews.push_back(Review { reviewer, score });

        env.storage().persistent().set(&key, &reviews);
    }

    pub fn get_reviews(env: Env, tenant: Address) -> Vec<Review> {
        let key = DataKey::Reviews(tenant);

        env.storage()
            .persistent()
            .get(&key)
            .unwrap_or(Vec::new(&env))
    }

    pub fn get_average_score(env: Env, tenant: Address) -> u32 {
        let reviews = Self::get_reviews(env, tenant);

        if reviews.len() == 0 {
            return 0;
        }

        let mut total: u32 = 0;

        for review in reviews.iter() {
            total += review.score;
        }

        total / reviews.len()
    }
}