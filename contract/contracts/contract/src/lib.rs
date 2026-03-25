#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec};

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Candidates,             // Lưu danh sách tên ứng viên
    Votes(String),          // Lưu số phiếu của từng người (Key là tên ứng viên)
    HasVoted(Address),      // Lưu trạng thái: Địa chỉ này đã bầu chưa?
}

#[contract]
pub struct ElectionContract;

#[contractimpl]
impl ElectionContract {
    
    // Hàm này để thiết lập danh sách ứng cử viên (ví dụ: ["Anh A", "Chị B"])
    pub fn initialize(env: Env, names: Vec<String>) {
        env.storage().instance().set(&DataKey::Candidates, &names);
        
        // Gán số phiếu ban đầu cho mỗi người là 0
        for name in names.iter() {
            env.storage().instance().set(&DataKey::Votes(name), &0u32);
        }
    }
    pub fn vote(env: Env, voter: Address, candidate: String) {
        // 1. Xác thực: Người bầu phải là người ký tên vào giao dịch này
        voter.require_auth();

        // 2. Kiểm tra: Nếu đã bầu rồi thì báo lỗi và dừng lại
        if env.storage().instance().has(&DataKey::HasVoted(voter.clone())) {
            panic!("Ban da bau roi, khong duoc bau nua!");
        }

        // 3. Lấy số phiếu hiện tại của ứng viên đó
        let current_votes: u32 = env.storage().instance()
            .get(&DataKey::Votes(candidate.clone()))
            .unwrap_or(0);

        // 4. Cộng thêm 1 phiếu và lưu lại
        env.storage().instance().set(&DataKey::Votes(candidate), &(current_votes + 1));

        // 5. Đánh dấu ví này đã bầu xong
        env.storage().instance().set(&DataKey::HasVoted(voter), &true);
    }
pub fn get_votes(env: Env, candidate: String) -> u32 {
        env.storage().instance()
            .get(&DataKey::Votes(candidate))
            .unwrap_or(0)
    }
} // Đóng ngoặc cuối cùng của block impl