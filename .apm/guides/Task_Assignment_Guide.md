# APM 0.5.3 - Hướng dẫn Giao việc
Hướng dẫn này định nghĩa cách Manager Agent phát hành phân công nhiệm vụ cho Implementation Agent và đánh giá việc hoàn thành của họ. Phân công nhiệm vụ điều phối công việc agent trong Task Loop của phiên APM, tuân theo Kế hoạch Triển khai.

## 1. Tổng quan Task Loop
Manager Agent phát hành Task Assignment Prompt → Người dùng chuyển cho Implementation Agent → Implementation Agent thực thi nhiệm vụ và ghi log công việc → Người dùng trả log cho Manager → Manager xem xét và xác định hành động tiếp theo (tiếp tục, theo dõi, ủy thác, hoặc cập nhật kế hoạch).

## 2. Định dạng Task Assignment Prompt
Task Assignment Prompt phải tương quan 1-1 với nhiệm vụ Kế hoạch Triển khai và bao gồm tất cả ngữ cảnh cần thiết cho thực thi thành công. Manager Agent phải phát hành các prompt này theo định dạng sau:

### 2.1. Kiểm tra Dependency
Trước khi tạo bất kỳ Task Assignment Prompt nào, kiểm tra dependency nhiệm vụ.

**Bước 1: Xác định Dependency**
Kiểm tra trường `Guidance` của nhiệm vụ Kế hoạch Triển khai cho khai báo dependency:
- `"Depends on: Task X.Y Output"` = Dependency cùng agent
- `"Depends on: Task X.Y Output by Agent Z"` = **DEPENDENCY XUYÊN-AGENT**

**Bước 2: Xác định Cách tiếp cận Tích hợp Ngữ cảnh**
- **Cùng Agent** (không có tag "by Agent X") → Sử dụng **Tham chiếu Ngữ cảnh Đơn giản** (Section 4.1)
- **Xuyên Agent** (có tag "by Agent X") → Sử dụng **Ngữ cảnh Tích hợp Toàn diện BẮT BUỘC** (Section 4.2)

### **Cảnh báo Dependency Xuyên-Agent**
**QUAN TRỌNG**: Dependency xuyên-agent yêu cầu Implementation Agent hoàn thành các bước đọc file và tích hợp chi tiết TRƯỚC KHI bắt đầu công việc nhiệm vụ chính.

### 2.2. Yêu cầu Giải thích Người dùng
Khi Người dùng yêu cầu giải thích cho các nhiệm vụ phức tạp sắp tới, Manager Agent nên bao gồm hướng dẫn giải thích chi tiết trong section `## Detailed Instructions` của Task Assignment Prompt.

**Giao thức Thời điểm Giải thích**:
- **Nhiệm vụ Single-Step**: Cung cấp giới thiệu cách tiếp cận ngắn gọn TRƯỚC thực thi, giải thích chi tiết SAU hoàn thành nhiệm vụ
- **Nhiệm vụ Multi-Step**: Áp dụng cùng mẫu cho mỗi bước - giới thiệu cách tiếp cận ngắn gọn TRƯỚC thực thi mỗi bước, giải thích chi tiết SAU hoàn thành mỗi bước

**Cách tiếp cận Tích hợp**: Thêm hướng dẫn giải thích như một phần của luồng thực thi nhiệm vụ, chỉ định:
- **Khía cạnh nào** cần giải thích chi tiết (cách tiếp cận kỹ thuật, lý do quyết định, tác động kiến trúc)
- **Phạm vi giải thích** cho các lĩnh vực kỹ thuật phức tạp
- **Yêu cầu thời điểm** tuân theo giao thức ở trên

**Triển khai**: Bao gồm hướng dẫn giải thích cùng với hướng dẫn nhiệm vụ bình thường trong section `## Detailed Instructions`. Sử dụng định dạng rõ ràng để phân biệt yêu cầu giải thích với yêu cầu thực thi. **Chỉ bao gồm hướng dẫn giải thích khi chúng được Người dùng yêu cầu rõ ràng.**

### 2.3. Cấu trúc Prompt với YAML Frontmatter
Chỉ bao gồm các section tùy chọn khi boolean front-matter của chúng là true

```markdown
---
task_ref: "Task <m.n> - Title"
agent_assignment: "Agent_<Domain>"
memory_log_path: "path/to/log/file"
execution_type: "single-step | multi-step"
dependency_context: true | false
ad_hoc_delegation: true | false
---

# APM Task Assignment: [Task Title]

## Task Reference
Implementation Plan: **Task X.Y - [Title]** assigned to **[Agent_<Domain>]**

## Context from Dependencies
[Chỉ bao gồm nếu dependency_context: true]
[Manager điền section này với hướng dẫn nội dung §4]

## Objective
[Mục tiêu nhiệm vụ một câu từ Kế hoạch Triển khai]

## Detailed Instructions
[Dựa trên subtask Kế hoạch Triển khai:]
- Cho nhiệm vụ single-step: "Hoàn thành tất cả mục trong một response"
- Cho nhiệm vụ multi-step: "Hoàn thành trong X trao đổi, một bước mỗi response. **CHỜ XÁC NHẬN NGƯỜI DÙNG** trước khi tiến sang mỗi bước tiếp theo."
- Chuyển đổi bullet subtask thành hướng dẫn có thể hành động chỉ định: làm gì, cách tiếp cận, triển khai ở đâu, và ràng buộc/thư viện nào sử dụng
- Bao gồm ngữ cảnh từ các trường Objective, Output, và Guidance của nhiệm vụ

## Expected Output
- Sản phẩm: [từ trường Output Kế hoạch Triển khai]
- Tiêu chí thành công: [định nghĩa hoàn thành rõ ràng]
- Vị trí file: [đường dẫn cụ thể cho file được tạo/sửa đổi]

## Memory Logging
Khi hoàn thành, bạn **PHẢI** ghi log công việc trong: `[memory_log_path]`
Tuân theo hướng dẫn .apm/guides/Memory_Log_Guide.md.

## Reporting Protocol
Sau khi ghi log, bạn **PHẢI** xuất code block **Final Task Report**.
- **Định dạng:** Sử dụng template chính xác được cung cấp trong hướng dẫn .claude/commands/apm-3-initiate-implementation.md của bạn.
- **Góc nhìn:** Viết từ góc nhìn Người dùng để họ có thể copy-paste lại cho Manager.

## Ad-Hoc Delegation
[Chỉ bao gồm nếu ad_hoc_delegation: true]
[Manager điền section này với hướng dẫn nội dung §7, bao gồm tham chiếu lệnh rõ ràng cho Debug/Research delegation (.claude/commands/apm-8-delegate-debug.md hoặc .claude/commands/apm-7-delegate-research.md)]
```

### 2.4. Định dạng Giao hàng
Trình bày Task Assignment Prompt như **một code block markdown đơn với YAML frontmatter ở đầu.** Điều này đảm bảo quy trình copy-paste mượt mà cho người dùng chuyển prompt giữa Manager và Implementation Agent.

## 3. Tích hợp Dependency Ngữ cảnh
Khi nhiệm vụ consumer phụ thuộc vào đầu ra producer ("Depends on: Task X.Y Output" trong Guidance Kế hoạch Triển khai), Manager cung cấp ngữ cảnh dựa trên phân công agent:

### 3.1. Dependency Cùng-Agent (Hướng dẫn Ngữ cảnh)
Khi **cùng Implementation Agent** làm việc trên cả nhiệm vụ producer và consumer:

**Cách tiếp cận Ngữ cảnh:**
- Cung cấp tham chiếu đầu ra cụ thể và chi tiết triển khai chính để nhớ lại
- Bao gồm vị trí file liên quan và artifact quan trọng đã tạo
- Giả định sự quen thuộc làm việc nhưng cung cấp hướng dẫn cụ thể cho tích hợp
- Mức chi tiết thay đổi dựa trên độ phức tạp dependency và khoảng thời gian giữa các nhiệm vụ

**Ví dụ Ngữ cảnh Cùng-Agent Đơn giản:**
```markdown
## Context from Dependencies
Dựa trên công việc Task 2.1 của bạn, sử dụng authentication middleware bạn đã tạo trong `src/middleware/auth.js` và các hàm JWT validation cho nhiệm vụ tích hợp frontend này.
```

**Ví dụ Ngữ cảnh Cùng-Agent Phức tạp:**
```markdown
## Context from Dependencies
Xây dựng trên triển khai API Task 2.3 của bạn:

**Đầu ra Chính để Sử dụng:**
- Authentication endpoint trong `src/api/auth.js` (POST /api/login, GET /api/verify)
- User validation middleware trong `src/middleware/auth.js`
- Cập nhật database schema trong `migrations/003_add_user_roles.sql`

**Chi tiết Triển khai để Nhớ lại:**
- JWT token bao gồm vai trò người dùng và quyền trong payload
- Xử lý lỗi trả về đối tượng lỗi chuẩn hóa với định dạng code/message
- Rate limiting áp dụng cho nỗ lực đăng nhập (triển khai trong middleware)

**Cách tiếp cận Tích hợp:**
Cho nhiệm vụ này, mở rộng hệ thống quyền role-based hiện có bạn đã xây dựng để xử lý yêu cầu admin dashboard mới.
```

#### Hướng dẫn Ngữ cảnh Cùng-Agent
- **Dependency Đơn giản**: Tham chiếu file và đầu ra chính với hướng dẫn tích hợp ngắn gọn
- **Dependency Phức tạp**: Bao gồm danh sách đầu ra chính, chi tiết triển khai quan trọng, và cách tiếp cận tích hợp rõ ràng
- **Cân nhắc Khoảng Thời gian**: Thêm chi tiết khi thời gian đáng kể trôi qua giữa các nhiệm vụ liên quan
- **Tham chiếu File**: Luôn bao gồm đường dẫn file cụ thể cho đầu ra cần sử dụng hoặc mở rộng
- **Tính liên tục Triển khai**: Nhấn mạnh xây dựng trên công việc trước thay vì bắt đầu mới

### 3.2. Dependency Xuyên-Agent (Ngữ cảnh Tích hợp Toàn diện)
Khi **Implementation Agent khác nhau** làm việc trên nhiệm vụ producer và consumer (Nhiệm vụ có tag "by Agent X"):

**Cách tiếp cận Ngữ cảnh Toàn diện:**
- Luôn cung cấp các bước tích hợp chi tiết với hướng dẫn đọc file rõ ràng
- Bao gồm tóm tắt đầu ra toàn diện và hướng dẫn sử dụng bất kể độ phức tạp dependency
- Cung cấp giao thức làm rõ Người dùng cho các điểm tích hợp mơ hồ
- Độ phức tạp chỉ ảnh hưởng đến lượng công việc tích hợp, không phải mức chi tiết được cung cấp

**Template Ngữ cảnh Xuyên-Agent:**

Từ mỗi section dưới đây sử dụng các tùy chọn phù hợp nhất với yêu cầu tích hợp ngữ cảnh cụ thể.
```markdown
## Context from Dependencies
Nhiệm vụ này [phụ thuộc vào/xây dựng trên/tích hợp với] [mô tả Task X.Y] được triển khai bởi [Producer_Agent]:

**Các bước Tích hợp (hoàn thành trong một response):**
1. [Đọc/Xem xét/Kiểm tra] [file/tài liệu cụ thể] tại [đường dẫn file] để hiểu [khía cạnh/chức năng cụ thể]
2. [Nghiên cứu/Phân tích] [file triển khai] trong [thư mục/đường dẫn file] để hiểu [cách tiếp cận kỹ thuật/cấu trúc dữ liệu/mẫu]
3. [Kiểm tra/Xem xét] [file test/ví dụ] tại [đường dẫn file] cho [mẫu sử dụng/hành vi mong đợi/ví dụ tích hợp]
4. [Các bước tích hợp bổ sung khi cần cho đầu ra cụ thể]

**Tóm tắt Đầu ra Producer:**
- [Chức năng/tính năng chính]: [Mô tả những gì đã xây dựng và cách hoạt động]
- [File/endpoint quan trọng]: [Vị trí và mục đích của đầu ra chính]
- [Cấu trúc dữ liệu/interface]: [Định dạng dữ liệu quan trọng, kiểu, hoặc hợp đồng]
- [Xử lý lỗi/validation]: [Cách xử lý lỗi và định dạng nào được sử dụng]
- [Bảo mật/xác thực]: [Bất kỳ biện pháp bảo mật hoặc yêu cầu xác thực nào]

**Yêu cầu Tích hợp:**
- [Yêu cầu cụ thể 1]: [Cách nhiệm vụ consumer phải tích hợp với đầu ra producer]
- [Yêu cầu cụ thể 2]: [Đặc tả tích hợp bổ sung]
- [Mẫu sử dụng]: [Cách sử dụng đúng đầu ra producer]
- [Ràng buộc/hạn chế]: [Hạn chế hoặc ràng buộc quan trọng cần xem xét]

**Giao thức Làm rõ Người dùng:**
Nếu [khía cạnh tích hợp cụ thể] mơ hồ sau khi hoàn thành các bước tích hợp, hỏi Người dùng về [lĩnh vực làm rõ cụ thể].
```

**Hướng dẫn Tạo Ngữ cảnh Xuyên-Agent:**
- **Luôn Toàn diện**: Bất kể độ phức tạp dependency, cung cấp các bước tích hợp đầy đủ, tóm tắt đầu ra, và yêu cầu chọn từ các tùy chọn phù hợp với yêu cầu dependency
- **Hướng dẫn Cụ thể File**: Luôn bao gồm đường dẫn file rõ ràng và tìm gì trong mỗi file
- **Bao phủ Đầu ra Hoàn chỉnh**: Ghi lại tất cả đầu ra liên quan, interface, và mẫu sử dụng từ nhiệm vụ producer
- **Yêu cầu Tích hợp**: Chỉ định chính xác cách nhiệm vụ consumer nên tích hợp với đầu ra producer
- **Giao thức Làm rõ**: Luôn bao gồm đường dẫn làm rõ Người dùng cho các điểm tích hợp mơ hồ
- **Giả định**: Agent Consumer không quen thuộc với công việc producer - giải thích mọi thứ cần thiết cho tích hợp thành công

### 3.3. Thực thi Tích hợp Ngữ cảnh
**Cho Dependency Cùng-Agent:**
- Không có section bước tích hợp riêng trong Task Assignment Prompt
- Bao gồm section "Context from Dependencies" tối thiểu với `dependency_context: true` trong YAML

**Cho Dependency Xuyên-Agent:**
- Bao gồm section "Context from Dependencies" chi tiết với `dependency_context: true` trong YAML
- Implementation Agent hoàn thành tất cả bước tích hợp trong một response trước nhiệm vụ chính

### 3.4. Hướng dẫn Tích hợp Ngữ cảnh cho Manager Agent

**Tạo Ngữ cảnh Cùng-Agent:**
- Xem xét Memory Log nhiệm vụ producer cho đầu ra và sản phẩm chính
- Tham chiếu công việc trước mà không lặp lại hướng dẫn chi tiết
- Tập trung vào kết nối đầu ra và tiếp tục công việc

**Tạo Ngữ cảnh Xuyên-Agent:**
- Xem xét kỹ Memory Log nhiệm vụ producer cho đầu ra, vị trí file, cách tiếp cận
- Tạo hướng dẫn đọc và xem xét file chi tiết
- Cung cấp tóm tắt đầu ra toàn diện và hướng dẫn sử dụng
- Bao gồm giao thức làm rõ Người dùng cho tích hợp phức tạp

## 4. Xem xét Memory Log
Khi Implementation Agent trả về, **xem xét Memory Log theo .apm/guides/Memory_Log_Guide.md section §5**. Đánh giá trạng thái hoàn thành nhiệm vụ, xác định blocker, và xác minh đầu ra khớp với kỳ vọng Kế hoạch Triển khai. Quét YAML frontmatter của log:
- Nếu `important_findings: true` hoặc `compatibility_issue: true`: Đọc các file nguồn hoặc đầu ra cụ thể được tham chiếu trong log để xác minh phát hiện. **Không tiến hành chỉ dựa trên nội dung log.**

## 5. Khung Hành động Tiếp theo
Dựa trên xem xét log, xác định bước tiếp theo phù hợp:

### 5.1. Tiếp tục Quy trình Làm việc
- Nhiệm vụ hoàn thành và thành công → Phát hành **Task Assignment Prompt tiếp theo** theo Kế hoạch Triển khai (Task Loop tiếp tục)
- Giai đoạn hoàn thành → **Tạo tóm tắt giai đoạn**, bắt đầu giai đoạn tiếp theo

### 5.2. Hành động Theo dõi
- Nhiệm vụ cần tinh chỉnh → Gửi **prompt theo dõi** sửa chữa cho cùng agent (nếu blocker kỹ thuật tiếp tục, xem xét **ủy thác Ad-Hoc trong prompt theo dõi**)
- Giả định kế hoạch không hợp lệ hoặc bất kỳ thay đổi nào khác cần thiết → **Cập nhật Kế hoạch Triển khai**

### 5.3. Tiêu chí Quyết định
- **Hoàn thành**: Tất cả sản phẩm được tạo ra, yêu cầu được đáp ứng
- **Một phần**: Một số tiến trình được thực hiện, vấn đề cụ thể được xác định
- **Bị chặn**: Không thể tiến hành mà không có đầu vào hoặc giải quyết bên ngoài

## 6. Giao thức Ủy thác Ad-Hoc
Đặt `ad_hoc_delegation: true` chỉ khi Kế hoạch Triển khai chứa các bước ủy thác rõ ràng cho nhiệm vụ.

### 6.1. Trách nhiệm Manager
Khi Kế hoạch Triển khai chứa các bước ủy thác rõ ràng, Manager Agent phải:
- Trích xuất yêu cầu ủy thác từ bước Kế hoạch Triển khai
- **Xác định loại ủy thác** (Debug, Research, hoặc khác) từ bước ủy thác Kế hoạch Triển khai
- **Bao gồm tham chiếu hướng dẫn rõ ràng** cho các loại ủy thác chuẩn trong Task Assignment Prompt nếu có thể
- Chỉ định những gì cần ủy thác và sản phẩm mong đợi trong prompt

**Tham chiếu Lệnh Ủy thác Chuẩn**:
- **Debug Delegation**: Tham chiếu .claude/commands/apm-8-delegate-debug.md
- **Research Delegation**: Tham chiếu .claude/commands/apm-7-delegate-research.md
- **Ủy thác Tùy chỉnh**: Tham chiếu file lệnh tùy chỉnh phù hợp nếu có

### 6.2. Yêu cầu Tích hợp
- Implementation Agent tạo prompt ủy thác và quản lý quy trình làm việc
- Agent Ad-hoc làm việc trong nhánh riêng được quản lý bởi Implementation Agent phân công; họ không ghi vào Memory
- Agent gốc tích hợp phát hiện và ghi log ủy thác trong khi Người dùng xóa phiên chat ủy thác (tùy chọn)

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
- **Task Assignment Prompt** (Prompt Giao việc – hướng dẫn chi tiết cho Implementation Agent)
- **Task Loop** (Vòng lặp Task – chu trình giao-thực thi-review)
- **Cross-Agent Dependency** (Phụ thuộc Liên Agent – task phụ thuộc output từ agent khác)
- **Context Integration** (Tích hợp Ngữ cảnh – cung cấp thông tin từ task trước)
- **Follow-Up Prompt** (Prompt Tiếp nối – yêu cầu bổ sung sau khi review log)

---

**Kết thúc Hướng dẫn**