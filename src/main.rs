use nalgebra::Vector4;
use nalgebra::DVector;

fn main() {
    // Walsh-Hadamard orthogonal spreading codes
    let code_user1 = Vector4::new(1.0,  1.0,  1.0,  1.0);
    let code_user2 = Vector4::new(1.0, -1.0,  1.0, -1.0);
    let code_user3 = Vector4::new(1.0,  1.0, -1.0, -1.0);

    // 1-bit user data
    let data_user1 =  1.0;
    let data_user2 = -1.0;
    let data_user3 =  1.0;

    // TX send

    // each user spreads their data before sending, more bandwidth is needed now
    // this is why CDMA is a essentially a frequency-spreading technique
    let signal_user1 = code_user1 * data_user1;
    let signal_user2 = code_user2 * data_user2;
    let signal_user3 = code_user3 * data_user3;

    // RX receive

    // received signal is a superposition (vector sum) of all individual user signals
    let received_signal = DVector::<f64>::zeros(4) + signal_user1 + signal_user2 + signal_user3;

    // RX decode, decide

    // leveraging orthogonality
    // spreading code eliminates everthing but related user data
    let decoded_user1 = received_signal.dot(&code_user1);
    let decoded_user2 = received_signal.dot(&code_user2);
    let decoded_user3 = received_signal.dot(&code_user3);

    // making a decision at receiver
    let data_out_user1 = if decoded_user1 >= 0.0 { 1.0 } else { -1.0 };
    let data_out_user2 = if decoded_user2 >= 0.0 { 1.0 } else { -1.0 };
    let data_out_user3 = if decoded_user3 >= 0.0 { 1.0 } else { -1.0 };

    assert!(data_user1 == data_out_user1);
    assert!(data_user2 == data_out_user2);
    assert!(data_user3 == data_out_user3);
}
