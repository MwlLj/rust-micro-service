use agent::services;

fn main() {
    services::serviceRegisterTest();
    services::getHealthServiceInfoTest("logs");
}
