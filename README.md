1. Project Title: Tên dự án
Voteryon - Hệ thống Bầu cử Minh bạch trên Blockchain Stellar
(Hoặc: Soroban Secure Vote)

2. Project Description: Mô tả dự án
Voteryon là một ứng dụng phi tập trung (dApp) chạy trên nền tảng hợp đồng thông minh Soroban (Stellar). Dự án giải quyết các vấn đề gian lận và thiếu minh bạch trong bầu cử truyền thống bằng cách ghi lại mỗi lá phiếu như một giao dịch không thể thay đổi trên blockchain.

Công nghệ sử dụng: Ngôn ngữ lập trình Rust, Soroban SDK, Stellar CLI.

Cơ chế: Sử dụng lưu trữ trạng thái (Contract Storage) để quản lý danh sách ứng viên và kiểm soát quyền biểu quyết của cử tri.

3. Project Vision: Tầm nhìn của dự án
Tạo ra một tiêu chuẩn mới cho việc bỏ phiếu điện tử: "Một người - Một phiếu - Vĩnh viễn công bằng". Tầm nhìn của dự án là loại bỏ hoàn toàn sự can thiệp của con người vào quá trình kiểm phiếu, giúp các tổ chức từ quy mô nhỏ (clb, công ty) đến quy mô lớn (đô thị) có thể tổ chức bầu cử với chi phí thấp và độ tin cậy tuyệt đối.

4. Key Features: Các tính năng chính
Xác thực danh tính (Identity Auth): Sử dụng hàm require_auth() để đảm bảo chỉ chủ sở hữu ví mới có thể bỏ phiếu bằng địa chỉ của mình.

Chống bầu cử kép (Anti-Double Voting): Cơ chế kiểm tra HasVoted ngăn chặn một địa chỉ ví bỏ phiếu nhiều hơn một lần.

Dữ liệu bất biến: Kết quả bầu cử được lưu trữ trên sổ cái (ledger) của Stellar, không ai có thể sửa đổi số liệu sau khi phiếu đã được nạp.

Truy vấn thời gian thực: Bất kỳ ai cũng có thể kiểm tra số phiếu hiện tại thông qua hàm get_votes mà không cần quyền quản trị.

5. Deployed Smart Contract Details: Chi tiết Hợp đồng
Contract ID: CCL5V6H5S7UUAS2IXJSJXZS422OTZODBK3TO4OD3AKM7QTT6ZVIEOXV2

Network: Stellar Testnet

Phương thức tương tác chính: initialize, vote, get_votes.

Ảnh chụp màn hình (Block Explorer):
Bạn hãy truy cập vào Stellar.Expert và chụp lại màn hình trang chi tiết hợp đồng này. Ảnh chụp nên bao gồm phần Contract Data và Recent Transactions để minh chứng hợp đồng đã hoạt động.

6. Future Scope: Phạm vi phát triển trong tương lai
Thời hạn bầu cử (Voting Deadline): Tích hợp thời gian thực của mạng lưới (env.ledger().timestamp()) để tự động đóng cổng bầu cử khi hết giờ.

Hệ thống cử tri đoàn (Whitelisting): Chỉ cho phép những địa chỉ ví nằm trong danh sách "Cử tri hợp lệ" được phép tham gia.

Quyền biểu quyết theo trọng số: Tính toán sức mạnh lá phiếu dựa trên số lượng token mà người đó nắm giữ (mô hình DAO).

Giao diện người dùng (Frontend): Xây dựng website hoàn chỉnh tích hợp ví Freighter để người dân không cần dùng dòng lệnh (CLI) vẫn có thể bỏ phiếu dễ dàng.
