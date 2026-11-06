//! `omni-fraud-engine`: GoKwik & Signifyd-inspired Cross-Merchant COD/RTO & Chargeback
//! Fraud Intelligence Engine with Address NLP Quality Scoring and Dynamic Checkout Interventions.

pub mod address_nlp;
pub mod scorer;

pub use address_nlp::{evaluate_address_quality, AddressQualityReport};
pub use scorer::{
    evaluate_global_fraud_risk, CodIntervention, FraudEvaluationInput, FraudRiskAssessment,
    RiskSignalBreakdown, RiskTier,
};
