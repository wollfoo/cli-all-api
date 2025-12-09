# APM 0.5.3 - Hướng dẫn Phân rã Dự án
Hướng dẫn này định nghĩa cách Setup Agent chuyển đổi phát hiện Tổng hợp Ngữ cảnh thành các phân rã nhiệm vụ có cấu trúc, được phân công agent. Tuân theo phương pháp luận hệ thống từ cấp cao đến chi tiết, nó ngăn chặn pattern matching thông qua sắp xếp quy trình làm việc chiến lược và chuyển đổi đầu ra chat-sang-file. Hướng dẫn đảm bảo độ chính xác phân rã nhiệm vụ cần thiết cho thành công của Implementation Agent trong khi giảm thiểu chi phí điều phối Manager Agent.

## 1. Tích hợp Ngữ cảnh & Tổng quan Phân rã

### 1.1. Insight Tổng hợp Ngữ cảnh Được giữ lại
Phân rã dự án chuyển đổi phát hiện Tổng hợp Ngữ cảnh thành phân rã nhiệm vụ có cấu trúc sử dụng **insight được giữ lại** từ giai đoạn khám phá. Những insight này cung cấp neo quyết định cụ thể và phải được tích hợp chủ động vào đặc tả nhiệm vụ:

**Insight Kỹ thuật & Phạm vi:**
- **Ranh giới domain** → Tạo phân công agent mạch lạc (xem §2.1)
- **Cờ độ phức tạp** → Tạo nhiệm vụ với độ chi tiết phù hợp (xem §4.1)
- **Dependency bên ngoài** → Lập kế hoạch hướng dẫn Người dùng cho hành động ngoài IDE (xem §4.1)
- **Nhu cầu điều tra** → Thêm bước Ủy thác Ad-Hoc tối thiểu một dòng khi cần trong nhiệm vụ nhiều bước bị ảnh hưởng (xem §4.2, §4.3)
- **Mẫu quy trình làm việc** → Tôn trọng tiến trình tự nhiên trong dependency (xem §4.5)

**Insight Quy trình & Triển khai:**
- **Tiêu chuẩn chất lượng và yêu cầu xác nhận** → Chuyển thành mục tiêu nhiệm vụ rõ ràng, tiêu chí chấp nhận, và bước xác nhận
- **Sở thích triển khai và phương pháp luận** → Chỉ định như cách tiếp cận thực thi nhiệm vụ bắt buộc và yêu cầu thủ tục
- **Ràng buộc quy trình và yêu cầu quy trình làm việc** → Nhúng như bước nhiệm vụ cụ thể, ràng buộc, và giao thức điều phối
- **Yêu cầu điều phối và theo dõi** → Cấu trúc như bước tương tác người dùng rõ ràng và điểm kiểm tra xem xét
- **Sở thích công cụ và ràng buộc kỹ thuật** → Chi tiết trong hướng dẫn nhiệm vụ như đặc tả kỹ thuật bắt buộc

**Xác minh Tích hợp:** Trong mỗi chu trình giai đoạn, kiểm tra để đảm bảo yêu cầu người dùng được nhấn mạnh xuất hiện như thành phần nhiệm vụ rõ ràng, không phải giả định nền.

### 1.2. Trình tự Phân rã Dự án
Setup Agent phải tuân theo tiến trình hệ thống từ cấp cao đến chi tiết này với cổng tiến trình bắt buộc và xác minh tích hợp:

Để duy trì hiệu quả, bạn phải thực thi **toàn bộ Trình tự Phân rã Dự án trong một response duy nhất**. Để ngăn pattern-matching và suy giảm chất lượng, bạn phải **XEN KẼ** phân tích của bạn:

1. **Phân tích Domain** (§2) → Phân công agent **trong chat**
2. **Định nghĩa Giai đoạn** (§3) → Trình tự giai đoạn **trong chat**
3. **Chu trình Giai đoạn** (§4) – **Trình tự Xen kẽ Nghiêm ngặt:** Cho mỗi giai đoạn, thực hiện **Phân tích Phase X hoàn chỉnh** trong chat: thực thi **Tích hợp Ngữ cảnh Giai đoạn & Xác định Nhiệm vụ** (§4.1), sau đó **Phân tích Hoàn chỉnh Nhiệm vụ Riêng lẻ** (§4.2) cho TẤT CẢ nhiệm vụ, sau đó **Đánh giá Dependency Giai đoạn** (§4.3).
   - **Chỉ sau khi** hoàn thành tất cả Phân tích Phase X trong chat, thêm nội dung Phase X vào file tuân theo **Thủ tục Tài liệu Giai đoạn** (§4.4).
   - **Sau đó và chỉ sau đó** chuyển sang Phase X+1 và lặp lại chu trình hoàn chỉnh.
   - **Lặp lại** trình tự xen kẽ nghiêm ngặt này cho tất cả giai đoạn mà không gộp hoặc bỏ qua ghi file **trừ khi được Người dùng chỉ định rõ ràng**.
4. **Xem xét Cuối cùng** (§5) → Tách agent (§5.1) + đánh dấu dependency xuyên-agent (§5.2) + **xác nhận yêu cầu quy trình trong file**
5. **Phê duyệt Kế hoạch** (§5.3) → Phê duyệt Người dùng dựa trên nội dung file + chat

**Cổng Tiến trình**: Mỗi bước phải hoàn thành trước khi tiến sang bước tiếp theo
**Xác minh Tích hợp**: Mỗi chu trình giai đoạn phải xác nhận rằng insight Tổng hợp Ngữ cảnh được tích hợp rõ ràng vào đặc tả nhiệm vụ

### 1.3. Mẫu Quy trình Chat-sang-File
Chuyển đổi ngữ cảnh chiến lược ngăn pattern matching:

**Hoạt động Chat**: Xác định domain, trình tự giai đoạn, phân rã nhiệm vụ mỗi giai đoạn, quyết định xem xét cuối cùng
**Hoạt động File**: Tài liệu mỗi chu trình giai đoạn hoàn thành, cập nhật tách agent, bổ sung dependency xuyên-agent
**Ngắt Ngữ cảnh**: Ghi file ngắt viết chat liên tục, cung cấp góc nhìn mới cho các giai đoạn tiếp theo do đó tránh pattern-matching

Định dạng file có cấu trúc (xem §4.4) ngăn hình thành template trong khi đảm bảo đầu ra sẵn sàng ngay lập tức cho Manager Agent sử dụng.

## 2. Phân tích Domain & Phân công Agent

### 2.1. Xác định Domain từ Ngữ cảnh Giữ lại
Chuyển đổi ranh giới domain được giữ lại từ Tổng hợp Ngữ cảnh thành các domain công việc logic yêu cầu mô hình tư duy và bộ kỹ năng khác nhau cho phân công Implementation Agent:

#### Tách biệt Lĩnh vực Kỹ năng
- Các lĩnh vực chuyên môn khác nhau được giữ lại → Agent riêng yêu cầu cơ sở kiến thức khác biệt
- Các môi trường kỹ thuật khác nhau được ghi nhận → Agent cụ thể domain cho mỗi tech stack
- Nhu cầu điều tra vs thực thi được xác định → Tách agent tập trung nghiên cứu vs tập trung triển khai
- Yêu cầu chuyên môn quy trình được xác định → Agent chuyên dụng cho hoạt động đảm bảo chất lượng, xác nhận, hoặc điều phối

#### Ranh giới Mô hình Tư duy
- Mẫu công việc hướng người dùng vs hướng hệ thống → Tách domain phía client vs phía server
- Luồng công việc sáng tạo vs phân tích → Ranh giới domain hướng nội dung vs hướng dữ liệu
- Hoạt động cấu hình vs phát triển → Domain agent tập trung thiết lập vs tập trung tính năng
- Quy trình thực thi vs xác nhận → Ranh giới domain tập trung triển khai vs tập trung xem xét

#### Tiêu chí Mạch lạc Domain
Đánh giá các domain tiềm năng theo yêu cầu mạch lạc cho thành công Implementation Agent:

**Yêu cầu Mô hình Tư duy Đơn lẻ:**
- Tất cả nhiệm vụ trong domain yêu cầu cách tiếp cận tư duy và phương pháp giải quyết vấn đề tương tự
- Phạm vi domain duy trì yêu cầu kiến thức kỹ thuật và bộ kỹ năng nhất quán
- Tiến trình nhiệm vụ trong domain tuân theo mẫu quy trình làm việc tự nhiên mà không chuyển đổi ngữ cảnh hoặc mô hình tư duy
- Yêu cầu quy trình phù hợp với chuyên môn domain và mẫu quy trình làm việc

**Nhóm Quy trình Làm việc Tự nhiên:**
- Nhiệm vụ trong domain xây dựng lẫn nhau một cách logic với dependency bên ngoài tối thiểu
- Ranh giới domain phù hợp với quan hệ quy trình làm việc được giữ lại từ Tổng hợp Ngữ cảnh
- Tiến trình công việc trong domain duy trì tính liên tục ngữ cảnh cho thực thi Implementation Agent
- Tiêu chuẩn chất lượng và yêu cầu xác nhận hỗ trợ tổ chức domain mạch lạc

**Xác nhận Ranh giới:**
- Tách biệt domain giảm chi phí điều phối Manager và tránh nhầm lẫn Implementation Agent
- Mỗi domain cung cấp giá trị độc lập trong khi hỗ trợ mục tiêu dự án tổng thể
- Ràng buộc quy trình và yêu cầu chất lượng được áp dụng nhất quán trong ranh giới domain

### 2.2. Tạo Đội Implementation Agent Ban đầu
Chuyển đổi các domain đã xác định thành phân công Implementation Agent ban đầu:

#### Quy trình Phân công
Trình bày đội agent hoàn chỉnh với lý do domain:
- Tạo một Implementation Agent cho mỗi domain logic được xác định từ phân tích §2.1
- Gán định danh agent mô tả phản ánh phạm vi domain: `Agent_<Domain>`
- Xem xét yêu cầu quy trình khi định nghĩa chuyên môn agent và nhu cầu điều phối
- Ước tính dependency xuyên-agent có khả năng (xem §5.2) và giảm thiểu thông qua ranh giới domain mạch lạc
- Lưu ý rằng xem xét phân phối khối lượng công việc xảy ra sau (xem §5.1) và có thể yêu cầu chia nhỏ agent

#### Hành động Chat Đầu tiên
Khi đọc hướng dẫn, ngay lập tức viết **trong chat** phân tích domain và phân công agent ban đầu trước khi tiến sang định nghĩa giai đoạn (xem §3). Điều này thiết lập nền tảng đội implementation agent cho phân công nhiệm vụ tiếp theo.

## 3. Định nghĩa Trình tự Giai đoạn

### 3.1. Xác định Giai đoạn từ Mẫu Quy trình Làm việc Giữ lại
Chuyển đổi mẫu quy trình làm việc được giữ lại từ Tổng hợp Ngữ cảnh thành cấu trúc tiến trình dự án logic:

#### Xác định Cấu trúc Giai đoạn
Sử dụng phạm vi và mẫu quy trình làm việc được giữ lại để xác định tổ chức giai đoạn phù hợp:

**Phân tích Mẫu Độ phức tạp:**
- Độ phức tạp phân lớp được đánh dấu → Giai đoạn phân cấp với dependency tiến trình
- Mẫu tuần tự được giữ lại → Giai đoạn tuyến tính tuân theo tiến trình quy trình làm việc tự nhiên
- Luồng công việc đồng thời được ghi nhận → Giai đoạn song song được tổ chức theo ranh giới domain hoặc thành phần
- Yêu cầu quy trình được xác định → Giai đoạn xác nhận, xem xét, hoặc đảm bảo chất lượng chuyên dụng khi ràng buộc quy trình làm việc yêu cầu

**Logic Bắt đầu-đến-Kết thúc:**
- Xác định yêu cầu khởi tạo dự án từ ngữ cảnh giữ lại
- Định nghĩa quy trình làm việc liên tục duy trì động lực giữa các giai đoạn
- Thiết lập tiêu chí hoàn thành và ranh giới sản phẩm cuối cùng
- Đảm bảo tiến trình dự án tự nhiên mà không dependency bị ép buộc
- Tích hợp ràng buộc quy trình và điểm kiểm tra chất lượng vào tiến trình giai đoạn

#### Đánh giá Ranh giới Giai đoạn
- Yêu cầu nghiên cứu mở rộng được xác định → Giai đoạn nghiên cứu chuyên dụng khi điều tra chặn công việc tiếp theo
- Yêu cầu kiểm tra và xác nhận được xác định → Giai đoạn xác nhận riêng hoặc điểm kiểm tra tích hợp
- Điểm nghẽn và mục đường dẫn quan trọng được giữ lại → Ranh giới giai đoạn tự nhiên tại ràng buộc dự án
- Hiểu phạm vi đơn giản → Tiến trình nhiệm vụ tuyến tính mà không tổ chức giai đoạn
- Tiêu chuẩn chất lượng và yêu cầu xem xét → Ranh giới giai đoạn bổ sung hoặc phạm vi giai đoạn mở rộng cho hoạt động xác nhận

#### Tiêu chí Phạm vi Giai đoạn
Đánh giá sự cần thiết và ranh giới giai đoạn theo yêu cầu dự án:
- Mỗi giai đoạn cung cấp giá trị độc lập hướng tới hoàn thành dự án
- Ranh giới giai đoạn phù hợp với quan hệ quy trình làm việc được giữ lại và điểm kiểm tra tự nhiên
- Tổ chức giai đoạn giảm độ phức tạp điều phối xuyên-agent
- Phạm vi giai đoạn hỗ trợ bảo toàn ngữ cảnh Implementation Agent trong các domain
- Yêu cầu quy trình và tiêu chuẩn chất lượng hỗ trợ tổ chức giai đoạn mạch lạc và quy trình làm việc xác nhận

### 3.2. Logic Tiến trình Giai đoạn
Chuyển đổi trình tự dự án đã định nghĩa trong §3.1 thành cấu trúc dự án phân giai đoạn:

#### Quy trình Trình bày
Trình bày trình tự giai đoạn đầy đủ với lý do hỗ trợ:
- Liệt kê giai đoạn theo thứ tự thực thi, cung cấp lý giải dựa trên mẫu quy trình làm việc giữ lại: `Phase X: <Phase_Name>`
- Ghi chú dependency giai đoạn và điểm bàn giao sản phẩm giữa các giai đoạn
- Xác nhận rằng tổ chức giai đoạn phù hợp với insight Tổng hợp Ngữ cảnh và yêu cầu dự án
- Đảm bảo ranh giới giai đoạn hỗ trợ tiến trình quy trình làm việc tự nhiên và giảm thiểu độ phức tạp điều phối xuyên-giai đoạn
- Xác nhận rằng yêu cầu quy trình và tiêu chuẩn chất lượng được tích hợp phù hợp vào cấu trúc giai đoạn
- Tiến sang thực thi chu trình giai đoạn (xem §4) tuân theo trình tự đã thiết lập

#### Hành động Chat Thứ hai
Sau khi trình bày phân công đội agent (xem §2.2), ngay lập tức viết **trong chat** phân tích trình tự giai đoạn trước khi bắt đầu chu trình giai đoạn (xem §4). Điều này thiết lập nền tảng cấu trúc dự án cho phân rã nhiệm vụ có hệ thống.

### 3.3. Khởi tạo Header Kế hoạch Triển khai
**BẮT BUỘC**: Trước khi tiến sang chu trình giai đoạn (xem §4), bạn **PHẢI** điền header của file `.apm/Implementation_Plan.md` được tạo bởi công cụ CLI `agentic-pm` sử dụng `apm init`.

File đã chứa template header với placeholder. Bạn phải:
1. **Đọc header hiện có** trong `.apm/Implementation_Plan.md`
2. **Điền tất cả trường header**:
   - Thay thế `<Project Name>` bằng tên dự án thực tế
   - Thay thế `[To be filled by Setup Agent before Project Breakdown]` trong trường **Last Modification** bằng: "Plan creation by the Setup Agent."
   - Thay thế `[To be filled by Setup Agent before Project Breakdown]` trong trường **Project Overview** bằng tóm tắt ngắn gọn về dự án
3. **Lưu header đã cập nhật** - Đây là hoạt động sửa file chuyên dụng phải hoàn thành trước khi bất kỳ nội dung giai đoạn nào được viết

**Chỉ sau khi header hoàn thành**, tiến sang chu trình giai đoạn (xem §4). Tất cả nội dung giai đoạn sẽ được thêm vào file này sau header.

## 4. Thực thi Chu trình Giai đoạn

### 4.1. Tích hợp Ngữ cảnh Giai đoạn & Xác định Nhiệm vụ
**Tuyên bố Tích hợp Ngữ cảnh**: Trước khi xác định nhiệm vụ, nêu rõ ràng **trong chat** insight giữ lại liên quan cho giai đoạn hiện tại: "Từ Tổng hợp Ngữ cảnh, tôi đã giữ lại [yêu cầu/ràng buộc/sở thích cụ thể]. Cho giai đoạn này, những điều này ảnh hưởng đến việc tạo nhiệm vụ bằng cách [cân nhắc cụ thể hoặc 'cung cấp ngữ cảnh dự án chung nhưng không có yêu cầu cấp nhiệm vụ trực tiếp']."

**Xác định Nhiệm vụ với Bảo vệ Chống Gói:**
Trong khi xác định nhiệm vụ cho giai đoạn này, áp dụng các bài kiểm tra này cho mỗi nhiệm vụ tiềm năng:

- **Kiểm tra Tập trung Đơn lẻ**: "Điều này có thể được hoàn thành bởi một agent trong một phiên làm việc tập trung mà không chuyển đổi ngữ cảnh/chế độ tư duy không?"
- **Kiểm tra Ranh giới Domain**: "Điều này có liên quan đến nhiều domain kỹ thuật hoặc bộ kỹ năng không liên quan không?"
- **Kiểm tra Giá trị Độc lập**: "Nếu tôi chia điều này thành các thành phần, mỗi thành phần có cung cấp giá trị độc lập không?"
- **Kiểm tra Sản phẩm Đơn vị Công việc Đơn lẻ**: "Hoàn thành nhiệm vụ này có dẫn đến sản phẩm có thể được hoàn thành như một đơn vị công việc duy nhất không?"
- **Kiểm tra Nhất quán Độ phức tạp**: "Độ phức tạp của nhiệm vụ này có khớp với các nhiệm vụ khác trong giai đoạn, hay nó phức tạp hơn đáng kể không?"

**Nếu bất kỳ bài kiểm tra nào gợi ý tách, tạo nhiệm vụ riêng biệt trong quá trình xác định.**

**Quy trình Xác định Nhiệm vụ**: Chuyển đổi mục tiêu giai đoạn thành nhiệm vụ tập trung sử dụng insight Tổng hợp Ngữ cảnh giữ lại. Áp dụng bảo vệ chống gói liên tục trong quá trình xác định. Mỗi nhiệm vụ nên cung cấp giá trị độc lập hướng tới hoàn thành giai đoạn. Không nhiệm vụ nào nên được gói nặng và chứa nhiều sản phẩm và mục tiêu.

**Trình bày Danh sách Nhiệm vụ**: Sau khi áp dụng bảo vệ, trình bày **trong chat** danh sách nhiệm vụ hoàn chỉnh cho giai đoạn: "Task X.1: [Name], Task X.2: [Name]..." trước khi tiến sang phân tích riêng lẻ.

**Kiểm tra Trước Ủy thác Ad-Hoc:** Trong khi liệt kê nhiệm vụ, nhanh chóng đánh dấu bất kỳ nhiệm vụ nào yêu cầu ủy thác ad-hoc dựa trên insight giữ lại. Sử dụng đánh dấu inline sau tên nhiệm vụ: "(ad-hoc: <mục đích>)". Giữ trong năm từ hoặc ít hơn; không lý giải ở đây.

### 4.2. Phân tích Hoàn chỉnh Nhiệm vụ Riêng lẻ
**QUAN TRỌNG**: Phân tích mỗi nhiệm vụ từ 4.1 riêng lẻ với lý luận hoàn chỉnh trước khi tiến sang nhiệm vụ tiếp theo. Không bao giờ xử lý gộp nhiều nhiệm vụ.**Cho mỗi nhiệm vụ được xác định, hoàn thành phân tích có hệ thống sau trong chat:**

```
#### **Task [X.Y]: [Tên Nhiệm vụ]**

**Phân tích Phạm vi:**
Nhiệm vụ này hoàn thành [mục tiêu cụ thể] và yêu cầu [phân tích phạm vi chi tiết]. Sản phẩm là [đầu ra hoặc artifact được định nghĩa rõ ràng].

**Đánh giá Thực thi:**
Phân tích những gì nhiệm vụ này yêu cầu:
- **Khả năng Agent**: Viết code, hoạt động file, lệnh terminal, cấu hình IDE, kiểm tra, tài liệu, hành động gọi công cụ
- **Điều phối Người dùng**: Nền tảng bên ngoài, xác thực tài khoản, cài đặt repository, cấu hình deploy, phê duyệt thiết kế, điểm kiểm tra phản hồi
- **Yêu cầu Hỗn hợp**: Tách các thành phần agent vs người dùng theo thứ tự logic

*Nêu đánh giá của bạn:* "Nhiệm vụ này yêu cầu [hành động agent cụ thể vs điều phối người dùng]. Bằng chứng cho thực thi agent: [khả năng IDE cụ thể]. Bằng chứng cho điều phối người dùng: [dependency bên ngoài, nhu cầu truy cập tài khoản]."

**Quyết định Phân loại:**
Đánh giá cấu trúc quy trình làm việc:
- **Tiêu chí single-step**: Công việc mạch lạc có thể hoàn thành trong một trao đổi, không có dependency nội bộ, không cần điểm xác nhận
- **Tiêu chí multi-step**: Dependency tuần tự nội bộ, nhu cầu xác nhận người dùng, nhu cầu ủy thác ad-hoc, yêu cầu xác nhận tiến trình, triển khai phức tạp với điểm dừng tự nhiên
- **Trường hợp biên**: Điều phối nền tảng bên ngoài = multi-step, nhu cầu nghiên cứu = multi-step với ủy thác ad-hoc, công việc kỹ thuật phức tạp với điểm dừng = multi-step

*Nêu lý luận của bạn:* "Task [X.Y] liên quan đến [mô tả quy trình làm việc]. Dựa trên [insight Tổng hợp Ngữ cảnh, yếu tố quy trình làm việc, nhu cầu xác nhận, dependency kỹ thuật], điều này yêu cầu thực thi [single/multi]-step vì [lý luận cụ thể]."

**Đặc tả Nội dung:**
Xác định nội dung nhiệm vụ phù hợp:
- **Biến đổi tự nhiên**: Số lượng cơ sở dựa trên độ phức tạp thực tế, không phải pattern matching
- **Hướng dẫn single-step**: Tối đa 4 bullet dựa trên độ phức tạp hướng dẫn
- **Hướng dẫn multi-step**: Tối đa 6 bước dựa trên dependency quy trình làm việc
- **Tập trung chất lượng**: Nội dung nên khớp với độ phức tạp nhiệm vụ riêng lẻ

*Lý giải lựa chọn của bạn:*
- **Nếu Single-step**: "Điều này cần [X] bullet vì [phân tích độ phức tạp]. Mỗi bullet giải quyết [nhu cầu hướng dẫn triển khai]."
- **Nếu Multi-step**: "Điều này cần [X] bước vì [phân tích dependency quy trình làm việc]. Mỗi bước đại diện cho [tiến trình tự nhiên]."

**Định nghĩa Nội dung:**
- Nếu được đánh dấu trong §4.1, đầu tiên thêm bước ủy thác ad-hoc: "Ad-Hoc Delegation – <mục đích>" (tùy chọn ref đến .claude/commands/apm-7-delegate-research.md hoặc .claude/commands/apm-8-delegate-debug.md), sau đó tiếp tục
- [Trình bày bullet hoặc bước thực tế với lý luận đã áp dụng]

**Phân tích Task [X.Y] hoàn thành** ← Nêu điều này trước khi tiến sang nhiệm vụ tiếp theo
```

**Lặp lại phân tích hoàn chỉnh này cho mỗi nhiệm vụ được xác định trong 4.1.**

### 4.3. Đánh giá Dependency Giai đoạn
**Sau khi hoàn thành phân tích riêng lẻ cho tất cả nhiệm vụ giai đoạn**, tiến hành xem xét dependency toàn diện:

**Xác định Dependency**: Tìm mẫu "phải làm A trước B" được giữ lại từ Tổng hợp Ngữ cảnh cho giai đoạn hiện tại. Xác định mối quan hệ producer-consumer thực sự giữa các nhiệm vụ được phân tích trong §4.2.

**Phân tích Dependency**: Định nghĩa dependency dựa trên yêu cầu quy trình làm việc thực tế và ràng buộc quy trình, không phải dependency nhân tạo. Bao gồm dependency quy trình như cổng chất lượng, yêu cầu xác nhận, và điểm kiểm tra xem xét.

**Trình bày Danh sách Dependency**: Trình bày **trong chat** danh sách dependency hoàn chỉnh với lý do sử dụng ký hiệu đơn giản: "Task X.Y phụ thuộc vào đầu ra Task Z.W vì [lý luận rõ ràng]"

### 4.4. Thủ tục Tài liệu Giai đoạn
**TRÌNH TỰ QUY TRÌNH LÀM VIỆC QUAN TRỌNG**: Hoàn thành TẤT CẢ phân tích nhiệm vụ riêng lẻ từ §4.2 và đánh giá dependency từ §4.3 trước bất kỳ hoạt động file nào.

#### Quy trình Tạo File
1. **Hoàn thành Phân tích Giai đoạn trong Chat Trước**: Trình bày tất cả phân tích nhiệm vụ riêng lẻ và dependency **trong chat** trước khi tiến sang tài liệu file
2. **Thời điểm Hoạt động File**: Thêm vào `Implementation_Plan.md` chỉ sau khi chu trình giai đoạn hoàn chỉnh được trình bày **trong chat**
3. **Hoạt động ghi đơn**: Mỗi chu trình giai đoạn dẫn đến **đúng một** file append chỉ chứa nội dung giai đoạn hiện tại

#### Định dạng Chuyển đổi Nội dung
Chuyển đổi phân tích riêng lẻ hoàn thành từ §4.2-4.3 thành định dạng file có cấu trúc, đảm bảo tất cả insight lý luận và yêu cầu quy trình được bảo toàn trong mô tả nhiệm vụ:

* **1. Header Tài liệu:** Header nên đã được điền từ §3.3. **KHÔNG** ghi đè hoặc sửa đổi header khi viết nội dung giai đoạn. Chỉ thêm section giai đoạn sau header hiện có.
* **2. Section Giai đoạn:** Sử dụng heading cấp 2: `## Phase <n>: <Name>`
* **3. Khối Nhiệm vụ:**
  - Sử dụng heading cấp 3: `### Task <n.m> – <Title> - <Agent_<Domain>>`
  - Trực tiếp dưới heading, thêm các trường meta này:
    - **Objective:** Mục tiêu nhiệm vụ một câu.
    - **Output:** Sản phẩm cụ thể (ví dụ: "Auth module files").
    - **Guidance:** Ràng buộc kỹ thuật hoặc cách tiếp cận chính. Hướng dẫn cho Manager Agent để phân công nhiệm vụ thành công.
* **4. Định dạng Nhiệm vụ Con:**
  - **Single-step**: Danh sách không thứ tự (`-`) cho hướng dẫn.
  - **Multi-step**: Danh sách có thứ tự (`1.`, `2.`) cho bước tuần tự.
  - **Nội dung**: Bước/bullet được suy ra trong Phân tích Chat của bạn (§4.2) với chi tiết bổ sung (nếu cần). Bảo toàn tất cả insight phân tích riêng lẻ, yêu cầu quy trình, và đặc tả triển khai từ phân rã chat
  - **Bước ủy thác Ad-Hoc:** tiền tố với `Ad-Hoc Delegation – <Purpose>` như một dòng đơn (tùy chọn ref hướng dẫn ngắn); không nội dung mở rộng trong file
* **5. Định dạng Dependency:** Thêm vào trường `Guidance` của Nhiệm vụ Consumer:
  - Cùng agent: `**Depends on: Task X.Y Output**`
  - Xuyên agent: `**Depends on: Task X.Y Output by Agent Z**`

## 5. Xem xét Cuối cùng & Tích hợp Xuyên-Agent

### 5.1. Đánh giá Khối lượng Công việc Agent & Tách Subdomain
Tiến hành xem xét toàn diện đầu tiên để đánh giá phân phối khối lượng công việc agent trên toàn bộ kế hoạch. Agent Quá tải (8+ nhiệm vụ) phải được chia nhỏ:

#### Đánh giá Khối lượng Công việc Agent
- Đếm tổng nhiệm vụ được giao cho mỗi agent trên tất cả giai đoạn hoàn thành
- Xác định agent với 8+ phân công nhiệm vụ yêu cầu chia nhỏ
- Xem xét phân phối nhiệm vụ cho tính mạch lạc logic trong domain agent và yêu cầu quy trình

#### Quy trình Tách Subdomain
Cho agent quá tải yêu cầu chia nhỏ:
- Phân tích nhiệm vụ trong domain agent cho ranh giới subdomain logic
- Tạo sub-agent mạch lạc dựa trên nhóm nhiệm vụ tự nhiên và nhu cầu chuyên môn quy trình: Agent_<Domain>_<Subdomain>
- Phân phối lại nhiệm vụ từ agent quá tải sang sub-agent phù hợp dựa trên ranh giới logic và yêu cầu triển khai
- Duy trì nguyên tắc mạch lạc domain từ §2.1 và căn chỉnh quy trình trong tách subdomain

#### Cập nhật File Tái phân công Agent
Cập nhật `Implementation_Plan.md` với phân công agent đã sửa đổi:
- Sửa đổi tất cả mục nhiệm vụ bị ảnh hưởng với phân công sub-agent mới
- Bảo toàn nội dung nhiệm vụ, dependency, định nghĩa hướng dẫn/bước, và đặc tả quy trình chính xác trong quá trình tái phân công
- Đảm bảo file phản ánh **phân công agent cuối cùng** trước khi tiến sang §5.2

### 5.2. Đánh dấu Dependency Xuyên-Agent
Tiến hành xem xét toàn diện thứ hai để xác định và đánh dấu dependency xuyên-agent sử dụng **phân công agent cuối cùng** từ §5.1:

#### Xác định Dependency Xuyên-Agent
- Xem xét toàn bộ kế hoạch với phân công agent cuối cùng để xác định dependency xuyên-agent
- Đánh dấu dependency là xuyên-agent chỉ nếu nhiệm vụ producer và consumer được giao cho agent khác nhau
- Nhiệm vụ với "Depends on Task X.Y" là xuyên-agent dependent nếu agent của Task X.Y ≠ agent của nhiệm vụ hiện tại
- Bao gồm dependency quy trình như xác nhận chất lượng, điểm kiểm tra xem xét, hoặc yêu cầu điều phối
- Trình bày tất cả dependency xuyên-agent được xác định **trong chat** trước khi tiến sang sửa file

#### Cập nhật File Ký hiệu Dependency
Cập nhật `Implementation_Plan.md` với ký hiệu dependency nâng cao:
- Thêm ký hiệu "by Agent Y" độc quyền cho dependency xuyên-agent
- Bảo toàn định dạng đơn giản "Depends on Task X.Y output" cho dependency cùng agent

### 5.3. Trình bày Kế hoạch Khái niệm & Phê duyệt Người dùng
Trình bày tổng quan kế hoạch và yêu cầu phê duyệt Người dùng dựa trên ngữ cảnh file và chat hoàn chỉnh:

#### Trình bày Tóm tắt Tổng quan
Trình bày **trong chat** thống kê kế hoạch cấp cao:
- Số lượng agent và domain
- Tổng giai đoạn với tên và số lượng nhiệm vụ
- Tổng số nhiệm vụ, và tổng số nhiệm vụ mỗi loại nhiệm vụ
- Số lượng dependency xuyên-agent
- Tóm tắt yêu cầu quy trình và đặc tả triển khai được tích hợp

#### Quy trình Xem xét & Phê duyệt Người dùng
- Hướng dẫn Người dùng xem xét kế hoạch có cấu trúc hoàn chỉnh trong `Implementation_Plan.md`
- Tham chiếu lý luận phân rã chi tiết từ trao đổi chat trước (§2-§4)
- Xác nhận rằng insight Tổng hợp Ngữ cảnh, bao gồm yêu cầu quy trình và tiêu chuẩn chất lượng, được phản ánh trong đặc tả nhiệm vụ
- Xử lý yêu cầu sửa đổi thông qua sửa đổi có mục tiêu đến section kế hoạch bị ảnh hưởng
- Lặp lại cho đến khi có phê duyệt rõ ràng từ Người dùng.

#### Định tuyến Bước Tiếp theo:
Khi kế hoạch được phê duyệt:
1. **Nếu Người dùng yêu cầu Xem xét Có hệ thống:** Tiến sang đọc .apm/guides/Project_Breakdown_Review_Guide.md`.
2. **Nếu Người dùng bỏ qua Xem xét:** Tiến trực tiếp đến **Manager Bootstrap Creation**.
   - **QUAN TRỌNG:** Bạn phải tạo Bootstrap Prompt sử dụng **TEMPLATE CHÍNH XÁC** được định nghĩa trong initiation prompt của bạn .claude/commands/apm-1-initiate-setup.md.
   - **Khôi phục Ngữ cảnh:** Nếu bạn không thể truy xuất template word-for-word từ ngữ cảnh của bạn, bạn phải **ĐỌC** file .claude/commands/apm-1-initiate-setup.md để làm mới bộ nhớ của bạn trước khi tạo prompt. Không xấp xỉ template.

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
- **Project Breakdown** (Phân rã Dự án – chia nhỏ yêu cầu thành tasks)
- **Domain Analysis** (Phân tích Lĩnh vực – xác định các vùng chuyên môn)
- **Phase Cycle** (Chu kỳ Giai đoạn – quy trình xử lý từng phase)
- **Agent Assignment** (Phân công Agent – giao task cho Implementation Agent)
- **Single-Step Task** (Nhiệm vụ Một bước – hoàn thành trong một response)
- **Multi-Step Task** (Nhiệm vụ Nhiều bước – cần xác nhận User qua từng bước)

---

**Kết thúc Hướng dẫn**