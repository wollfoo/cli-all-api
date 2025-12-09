# APM 0.5.3 - Hướng dẫn Hệ thống Memory
Hướng dẫn này giải thích cách các phiên APM lưu trữ và phát triển memory sử dụng hệ thống **Dynamic-MD** (Định dạng Markdown Động).

Nhiệm vụ Memory được giao cho *Manager Agent* - người duy trì hệ thống. Chi tiết về các file Memory Log riêng lẻ nằm trong .apm/guides/Memory_Log_Guide.md.

## 1  Tổng quan Hệ thống Memory
Hệ thống Memory Dynamic-MD tổ chức memory với cấu trúc sau:

- **Bố cục lưu trữ:** Thư mục `.apm/Memory/` + `Memory_Root.md` + các thư mục con `Phase_XX_<slug>/` trong thư mục `.apm/`
- **Định dạng log:** Một `Task_XX_<slug>.md` Memory Log cho mỗi nhiệm vụ
- **Tóm tắt:** Sau khi mỗi giai đoạn hoàn thành, một subsection inline được thêm vào file `Memory_Root.md` tóm tắt giai đoạn

**Memory Log** (Nhật ký Bộ nhớ) ghi lại ngữ cảnh chi tiết cấp nhiệm vụ và được viết bởi Implementation Agent sau mỗi lần hoàn thành nhiệm vụ. Xem .apm/guides/Memory_Log_Guide.md cho schema và quy tắc viết.

## 2  Trách nhiệm Manager Agent
Trách nhiệm chính của Manager Agent khi duy trì Hệ thống Memory trong phiên APM:

1. **Khởi tạo Header Memory Root (Chỉ Phiên Đầu tiên)**: Trước khi bắt đầu thực thi giai đoạn đầu tiên, điền header của `.apm/Memory/Memory_Root.md`. File được tạo sẵn bởi công cụ CLI `agentic-pm` sử dụng `apm init`, với template header chứa các placeholder. Thay thế tất cả placeholder bằng giá trị thực tế trước khi tiến hành thực thi giai đoạn.

2. Giữ cấu trúc Hệ thống Memory (thư mục/log) đồng bộ với Kế hoạch Triển khai hiện tại. Cập nhật khi Giai đoạn hoặc Nhiệm vụ thay đổi.

3. Sau mỗi giai đoạn, tạo và thêm tóm tắt ngắn gọn tham chiếu các Memory Log liên quan.

### Quản lý Giai đoạn và Nhiệm vụ
**Lưu ý**: Header Memory Root phải được điền trước khi thực thi giai đoạn đầu tiên bắt đầu (xem trách nhiệm #1 ở trên).

1. Khi vào giai đoạn, tạo `.apm/Memory/Phase_XX_<slug>/` nếu thiếu. Cho mỗi nhiệm vụ trong giai đoạn, tạo một Memory Log **hoàn toàn trống**, tuân theo .apm/guides/Memory_Log_Guide.md:
    - `Task_Y_Z_<slug>.md`

**Tất cả Memory Log cho giai đoạn hiện tại phải được tạo TRƯỚC Task Assignment Prompt đầu tiên cho mỗi nhiệm vụ.**
**Sử dụng task ID và tiêu đề từ Kế hoạch Triển khai (loại trừ phân công agent).**
**Ví dụ: Task "Task 2.1 - Deploy Updates | Agent_Backend" → `Task_2_1_Deploy_Updates.md`**

2. Sau mỗi lần thực thi nhiệm vụ, xem xét Memory Log **được điền bởi Implementation Agent**, được cung cấp qua Người dùng.
   - Nếu log chứa `important_findings: true` hoặc `compatibility_issues: true`, bạn **PHẢI** kiểm tra các file/artifact đầu ra được tham chiếu để xác nhận phát hiện trước khi đưa ra quyết định.
   

3. Vào cuối giai đoạn, thêm tóm tắt vào `.apm/Memory/Memory_Root.md`:
    ```markdown
    ## Phase XX – <Tên Giai đoạn> Tóm tắt 
    * Tóm tắt kết quả (≤ 200 từ)
    * Danh sách các Agent tham gia
    * Liên kết đến tất cả task log của giai đoạn
    ```
    Giữ tóm tắt ≤30 dòng.

---

## Quy tắc Ngôn ngữ – BẮT BUỘC

### Nguyên tắc cốt lõi
- **BẮT BUỘC**: Phản hồi bằng **tiếng Việt**.
- **GIẢI THÍCH THUẬT NGỮ**: Mọi thuật ngữ tiếng Anh phải kèm mô tả tiếng Việt.

### Cú pháp chuẩn
`**<English Term>** (mô tả tiếng Việt – chức năng/mục đích)`

### Code Comments / Documentation / Logs
- **Mặc định**: Comments, logs, docs, docstrings viết bằng tiếng Việt.
- **Song ngữ tại điểm quan trọng**: Dòng đầu tiếng Việt, dòng sau tiếng Anh.
- **Structured logging**: Keys bằng tiếng Anh (machine parsing), messages bằng tiếng Việt.
- **Ngoại lệ**: Khi library/standard yêu cầu tiếng Anh, thêm ghi chú tiếng Việt.

### Ví dụ
- **Memory System** (Hệ thống Bộ nhớ – quản lý ngữ cảnh dự án)
- **Memory Root** (Gốc Bộ nhớ – tệp tổng hợp chính)
- **Phase Summary** (Tóm tắt Giai đoạn – kết quả sau mỗi phase)
- **Storage Layout** (Cấu trúc Lưu trữ – tổ chức thư mục Memory)

---

**Kết thúc Hướng dẫn**