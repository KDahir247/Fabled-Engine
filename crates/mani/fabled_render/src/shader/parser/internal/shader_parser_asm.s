# --------------- Shader Parser Dissassembly -------------------
00007FF6A7262075  add          rbx, 46h
00007FF6A7262079  je           static void Fabled_Engine::main+17Ah (00007FF6A72620BAh)
00007FF6A726207B  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A7262082  test         rcx, rcx
00007FF6A7262085  jne          static void Fabled_Engine::main+15Fh (00007FF6A726209Fh)
00007FF6A7262087  call         GetProcessHeap (00007FF6A735A4E2h)
00007FF6A726208C  test         rax, rax
00007FF6A726208F  je           static void Fabled_Engine::main+509Ch (00007FF6A7266FDCh)
00007FF6A7262095  mov          rcx, rax
00007FF6A7262098  mov          qword ptr [00007FF6A73861D8h], rax
00007FF6A726209F  xor          edx, edx
00007FF6A72620A1  mov          r8, rbx
00007FF6A72620A4  call         HeapAlloc (00007FF6A735A4E8h)
00007FF6A72620A9  test         rax, rax
00007FF6A72620AC  je           static void Fabled_Engine::main+509Ch (00007FF6A7266FDCh)
00007FF6A72620B2  mov          r13, rax
00007FF6A72620B5  mov          r14, rbx
00007FF6A72620B8  jmp          static void Fabled_Engine::main+180h (00007FF6A72620C0h)
00007FF6A72620BA  mov          r13d, 1h
00007FF6A72620C0  mov          rcx, r13
00007FF6A72620C3  mov          rdx, rdi
00007FF6A72620C6  mov          r8, rbx
00007FF6A72620C9  call         memcpy (00007FF6A735B5C7h)
00007FF6A72620CE  lea          rdi, [rsp+610h]
00007FF6A72620D6  mov          rcx, rdi
00007FF6A72620D9  mov          rdx, r13
00007FF6A72620DC  mov          r8, rbx
00007FF6A72620DF  call         static void std::path::Path::components (00007FF6A734F6E0h)
00007FF6A72620E4  lea          rcx, [rsp+3C0h]
00007FF6A72620EC  mov          rdx, rdi
00007FF6A72620EF  call         static void std::path::{{impl}}::next_back (00007FF6A73559D0h)
00007FF6A72620F4  mov          rax, qword ptr [rsp+3C0h]
00007FF6A72620FC  cmp          rax, 5h
00007FF6A7262100  je           static void Fabled_Engine::main+243h (00007FF6A7262183h)
00007FF6A7262106  cmp          eax, 4h
00007FF6A7262109  jne          static void Fabled_Engine::main+243h (00007FF6A7262183h)
00007FF6A726210B  mov          rbp, qword ptr [rsp+3C8h]
00007FF6A7262113  mov          rdx, qword ptr [rsp+3D0h]
00007FF6A726211B  cmp          rdx, 2h
00007FF6A726211F  jne          static void Fabled_Engine::main+1ECh (00007FF6A726212Ch)
00007FF6A7262121  movzx        eax, word ptr [rbp]
00007FF6A7262125  cmp          eax, 2E2Eh
00007FF6A726212A  je           static void Fabled_Engine::main+243h (00007FF6A7262183h)
00007FF6A726212C  lea          rcx, [rbp-1h]
00007FF6A7262130  not          rbp
00007FF6A7262133  xor          eax, eax
00007FF6A7262135  nop          word ptr cs:[rax+rax*1], ax
00007FF6A726213F  nop
00007FF6A7262140  cmp          rdx, rax
00007FF6A7262143  je           static void Fabled_Engine::main+243h (00007FF6A7262183h)
00007FF6A7262145  mov          rsi, rax
00007FF6A7262148  add          rbp, 1h
00007FF6A726214C  cmp          byte ptr [rcx+rdx*1], 2Eh
00007FF6A7262150  lea          rcx, [rcx-1h]
00007FF6A7262154  lea          rax, [rax+1h]
00007FF6A7262158  jne          static void Fabled_Engine::main+200h (00007FF6A7262140h)
00007FF6A726215A  mov          rcx, rdx
00007FF6A726215D  sub          rcx, rax
00007FF6A7262160  add          rcx, 1h
00007FF6A7262164  cmp          rdx, rcx
00007FF6A7262167  jb           static void Fabled_Engine::main+52A0h (00007FF6A72671E0h)
00007FF6A726216D  mov          rcx, rdx
00007FF6A7262170  sub          rcx, rbp
00007FF6A7262173  xor          r15d, r15d
00007FF6A7262176  cmp          rdx, rax
00007FF6A7262179  cmovne       r15, rcx
00007FF6A726217D  jne          static void Fabled_Engine::main+BC6h (00007FF6A7262B06h)
00007FF6A7262183  lea          rcx, [rsp+610h]
00007FF6A726218B  call         static void std::backtrace::Backtrace::capture (00007FF6A7353840h)
00007FF6A7262190  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A7262197  test         rcx, rcx
00007FF6A726219A  jne          static void Fabled_Engine::main+274h (00007FF6A72621B4h)
00007FF6A726219C  call         GetProcessHeap (00007FF6A735A4E2h)
00007FF6A72621A1  test         rax, rax
00007FF6A72621A4  je           static void Fabled_Engine::main+5071h (00007FF6A7266FB1h)
00007FF6A72621AA  mov          rcx, rax
00007FF6A72621AD  mov          qword ptr [00007FF6A73861D8h], rax
00007FF6A72621B4  mov          r8d, 50h
00007FF6A72621BA  xor          edx, edx
00007FF6A72621BC  call         HeapAlloc (00007FF6A735A4E8h)
00007FF6A72621C1  test         rax, rax
00007FF6A72621C4  je           static void Fabled_Engine::main+5071h (00007FF6A7266FB1h)
00007FF6A72621CA  mov          rbx, rax
00007FF6A72621CD  lea          rax, [00007FF6A7369048h]
00007FF6A72621D4  mov          qword ptr [rbx], rax
00007FF6A72621D7  movdqu       xmm0, xmmword ptr [rsp+610h]
00007FF6A72621E0  movups       xmm1, xmmword ptr [rsp+620h]
00007FF6A72621E8  movups       xmm2, xmmword ptr [rsp+630h]
00007FF6A72621F0  movdqu       xmmword ptr [rbx+8h], xmm0
00007FF6A72621F5  movups       xmmword ptr [rbx+18h], xmm1
00007FF6A72621F9  movups       xmmword ptr [rbx+28h], xmm2
00007FF6A72621FD  mov          rax, qword ptr [rsp+640h]
00007FF6A7262205  mov          qword ptr [rbx+38h], rax
00007FF6A7262209  lea          rax, [00007FF6A7368C20h]
00007FF6A7262210  mov          qword ptr [rbx+40h], rax
00007FF6A7262214  mov          qword ptr [rbx+48h], 25h
00007FF6A726221C  mov          sil, 1h
00007FF6A726221F  test         r14, r14
00007FF6A7262222  je           static void Fabled_Engine::main+2F5h (00007FF6A7262235h)
00007FF6A7262224  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A726222B  xor          edx, edx
00007FF6A726222D  mov          r8, r13
00007FF6A7262230  call         HeapFree (00007FF6A735A4DCh)
00007FF6A7262235  test         sil, sil
00007FF6A7262238  jne          static void Fabled_Engine::main+4EEAh (00007FF6A7266E2Ah)
00007FF6A726223E  mov          qword ptr [rsp+610h], rbx
00007FF6A7262246  movaps       xmm0, xmmword ptr [rsp+3C0h]
00007FF6A726224E  movaps       xmm1, xmmword ptr [rsp+3D0h]
00007FF6A7262256  movaps       xmm2, xmmword ptr [rsp+3E0h]
00007FF6A726225E  movdqa       xmm3, xmmword ptr [rsp+3F0h]
00007FF6A7262267  movups       xmmword ptr [rsp+618h], xmm0
00007FF6A726226F  movups       xmmword ptr [rsp+628h], xmm1
00007FF6A7262277  movups       xmmword ptr [rsp+638h], xmm2
00007FF6A726227F  movdqu       xmmword ptr [rsp+648h], xmm3
00007FF6A7262288  movaps       xmm0, xmmword ptr [rsp+400h]
00007FF6A7262290  movups       xmmword ptr [rsp+658h], xmm0
00007FF6A7262298  movaps       xmm0, xmmword ptr [rsp+410h]
00007FF6A72622A0  movups       xmmword ptr [rsp+668h], xmm0
00007FF6A72622A8  movaps       xmm0, xmmword ptr [rsp+420h]
00007FF6A72622B0  movups       xmmword ptr [rsp+678h], xmm0

// encode_shader
00007FF6A72626E3  mov          rdi, qword ptr [rsp+3A8h]
00007FF6A72626EB  mov          r8, qword ptr [rsp+3B8h]
00007FF6A72626F3  mov          rsi, qword ptr [rsp+3B0h]
00007FF6A72626FB  mov          dword ptr [rsp+3E0h], 0h
00007FF6A7262706  mov          word ptr [rsp+3E4h], 0h
00007FF6A7262710  mov          dword ptr [rsp+3D4h], 0h
00007FF6A726271B  xorps        xmm0, xmm0
00007FF6A726271E  movaps       xmmword ptr [rsp+3C0h], xmm0
00007FF6A7262726  mov          qword ptr [rsp+3D8h], 7h
00007FF6A7262732  mov          byte ptr [rsp+3E1h], 1h
00007FF6A726273A  mov          byte ptr [rsp+3E4h], 1h
00007FF6A7262742  mov          byte ptr [rsp+3E3h], 1h
00007FF6A726274A  lea          rcx, [rsp+250h]
00007FF6A7262752  lea          r9, [rsp+3C0h]
00007FF6A726275A  mov          rdx, rdi
00007FF6A726275D  call         static void std::sys::windows::fs::File::open (00007FF6A7354AC0h)
00007FF6A7262762  mov          rbx, qword ptr [rsp+250h]
00007FF6A726276A  mov          rbp, qword ptr [rsp+258h]
00007FF6A7262772  mov          r13, qword ptr [rsp+260h]
00007FF6A726277A  test         rsi, rsi
00007FF6A726277D  je           static void Fabled_Engine::main+850h (00007FF6A7262790h)
00007FF6A726277F  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A7262786  xor          edx, edx
00007FF6A7262788  mov          r8, rdi
00007FF6A726278B  call         HeapFree (00007FF6A735A4DCh)
00007FF6A7262790  test         rbx, rbx
00007FF6A7262793  jne          static void Fabled_Engine::main+4FC0h (00007FF6A7266F00h)
00007FF6A7262799  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A72627A0  test         rcx, rcx
00007FF6A72627A3  jne          static void Fabled_Engine::main+87Dh (00007FF6A72627BDh)
00007FF6A72627A5  call         GetProcessHeap (00007FF6A735A4E2h)
00007FF6A72627AA  test         rax, rax
00007FF6A72627AD  je           static void Fabled_Engine::main+4EC8h (00007FF6A7266E08h)
00007FF6A72627B3  mov          rcx, rax
00007FF6A72627B6  mov          qword ptr [00007FF6A73861D8h], rax
00007FF6A72627BD  mov          r8d, 2000h
00007FF6A72627C3  xor          edx, edx
00007FF6A72627C5  call         HeapAlloc (00007FF6A735A4E8h)
00007FF6A72627CA  test         rax, rax
00007FF6A72627CD  je           static void Fabled_Engine::main+4EC8h (00007FF6A7266E08h)
00007FF6A72627D3  mov          qword ptr [rsp+250h], 1h
00007FF6A72627DF  mov          qword ptr [rsp+258h], rbp
00007FF6A72627E7  mov          qword ptr [rsp+260h], rax
00007FF6A72627EF  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000002000 (00007FF6A73685B0h)]
00007FF6A72627F6  movups       xmmword ptr [rsp+268h], xmm0
00007FF6A72627FE  mov          byte ptr [rsp+278h], 0h
00007FF6A7262806  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A726280D  test         rcx, rcx
00007FF6A7262810  jne          static void Fabled_Engine::main+8EAh (00007FF6A726282Ah)
00007FF6A7262812  call         GetProcessHeap (00007FF6A735A4E2h)
00007FF6A7262817  test         rax, rax
00007FF6A726281A  je           static void Fabled_Engine::main+4ED9h (00007FF6A7266E19h)
00007FF6A7262820  mov          rcx, rax
00007FF6A7262823  mov          qword ptr [00007FF6A73861D8h], rax
00007FF6A726282A  mov          edi, 95h
00007FF6A726282F  mov          r8d, 95h
00007FF6A7262835  xor          edx, edx
00007FF6A7262837  call         HeapAlloc (00007FF6A735A4E8h)
00007FF6A726283C  test         rax, rax
00007FF6A726283F  je           static void Fabled_Engine::main+4ED9h (00007FF6A7266E19h)
00007FF6A7262845  mov          rbp, rax
00007FF6A7262848  lea          rdx, [00007FF6A7368E30h]
00007FF6A726284F  mov          r8d, 95h
00007FF6A7262855  mov          rcx, rax
00007FF6A7262858  call         memcpy (00007FF6A735B5C7h)
00007FF6A726285D  mov          qword ptr [rsp+1D0h], rbp
00007FF6A7262865  movaps       xmm0, xmmword ptr [__xmm@00000000000000950000000000000095 (00007FF6A73685C0h)]
00007FF6A726286C  movups       xmmword ptr [rsp+1D8h], xmm0
00007FF6A7262874  test         r15, r15
00007FF6A7262877  jne          static void Fabled_Engine::main+B14h (00007FF6A7262A54h)
00007FF6A726287D  lea          rcx, [rdi+rbp*1]
00007FF6A7262881  mov          rdx, r12
00007FF6A7262884  mov          r8, r15
00007FF6A7262887  call         memcpy (00007FF6A735B5C7h)
00007FF6A726288C  add          rdi, r15
00007FF6A726288F  mov          qword ptr [rsp+1E0h], rdi
00007FF6A7262897  mov          rax, qword ptr [rsp+268h]
00007FF6A726289F  mov          rsi, qword ptr [rsp+270h]
00007FF6A72628A7  sub          rax, rsi
00007FF6A72628AA  cmp          rax, rdi
00007FF6A72628AD  jbe          static void Fabled_Engine::main+B3Eh (00007FF6A7262A7Eh)
00007FF6A72628B3  mov          rcx, qword ptr [rsp+260h]
00007FF6A72628BB  add          rcx, rsi
00007FF6A72628BE  mov          rdx, rbp
00007FF6A72628C1  mov          r8, rdi
00007FF6A72628C4  call         memcpy (00007FF6A735B5C7h)
00007FF6A72628C9  add          rsi, rdi
00007FF6A72628CC  mov          qword ptr [rsp+270h], rsi
00007FF6A72628D4  lea          rcx, [rsp+250h]
00007FF6A72628DC  call         static union enum$<core::result::Result<tuple<>, std::io::error::Error>, 0, 3, Err> std::io::buffered::bufwriter::BufWriter<std::fs::File>::flush_buf<std::fs::File> (00007FF6A7267760h)
00007FF6A72628E1  mov          rbx, rax
00007FF6A72628E4  cmp          bl, 4h
00007FF6A72628E7  jne          static void Fabled_Engine::main+4FFEh (00007FF6A7266F3Eh)
00007FF6A72628ED  cmp          dword ptr [rsp+250h], 1h
00007FF6A72628F5  jne          static void Fabled_Engine::main+5082h (00007FF6A7266FC2h)
00007FF6A72628FB  cmp          qword ptr [rsp+1D8h], 0h
00007FF6A7262904  je           static void Fabled_Engine::main+9D7h (00007FF6A7262917h)
00007FF6A7262906  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A726290D  xor          edx, edx
00007FF6A726290F  mov          r8, rbp
00007FF6A7262912  call         HeapFree (00007FF6A735A4DCh)
00007FF6A7262917  mov          rax, qword ptr [rsp+250h]
00007FF6A726291F  cmp          rax, 1h
00007FF6A7262923  jne          static void Fabled_Engine::main+A4Ch (00007FF6A726298Ch)
00007FF6A7262925  cmp          byte ptr [rsp+278h], 0h
00007FF6A726292D  jne          static void Fabled_Engine::main+A4Ch (00007FF6A726298Ch)
00007FF6A726292F  lea          rcx, [rsp+250h]
00007FF6A7262937  call         static union enum$<core::result::Result<tuple<>, std::io::error::Error>, 0, 3, Err> std::io::buffered::bufwriter::BufWriter<std::fs::File>::flush_buf<std::fs::File> (00007FF6A7267760h)
00007FF6A726293C  cmp          al, 3h
00007FF6A726293E  jne          static void Fabled_Engine::main+A44h (00007FF6A7262984h)
00007FF6A7262940  mov          rdi, rdx
00007FF6A7262943  mov          rcx, qword ptr [rdx]
00007FF6A7262946  mov          rax, qword ptr [rdx+8h]
00007FF6A726294A  call         qword ptr [rax]
00007FF6A726294C  mov          rax, qword ptr [rdi+8h]
00007FF6A7262950  cmp          qword ptr [rax+8h], 0h
00007FF6A7262955  je           static void Fabled_Engine::main+A33h (00007FF6A7262973h)
00007FF6A7262957  mov          r8, qword ptr [rdi]
00007FF6A726295A  cmp          qword ptr [rax+10h], 11h
00007FF6A726295F  jb           static void Fabled_Engine::main+A25h (00007FF6A7262965h)
00007FF6A7262961  mov          r8, qword ptr [r8-8h]
00007FF6A7262965  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A726296C  xor          edx, edx
00007FF6A726296E  call         HeapFree (00007FF6A735A4DCh)
00007FF6A7262973  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A726297A  xor          edx, edx
00007FF6A726297C  mov          r8, rdi
00007FF6A726297F  call         HeapFree (00007FF6A735A4DCh)
00007FF6A7262984  mov          rax, qword ptr [rsp+250h]
00007FF6A726298C  test         rax, rax
00007FF6A726298F  je           static void Fabled_Engine::main+A5Eh (00007FF6A726299Eh)
00007FF6A7262991  mov          rcx, qword ptr [rsp+258h]
00007FF6A7262999  call         CloseHandle (00007FF6A735A4F4h)
00007FF6A726299E  cmp          qword ptr [rsp+268h], 0h
00007FF6A72629A7  je           static void Fabled_Engine::main+A7Fh (00007FF6A72629BFh)
00007FF6A72629A9  mov          r8, qword ptr [rsp+260h]
00007FF6A72629B1  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A72629B8  xor          edx, edx
00007FF6A72629BA  call         HeapFree (00007FF6A735A4DCh)
00007FF6A72629BF  test         r14, r14
00007FF6A72629C2  je           static void Fabled_Engine::main+A95h (00007FF6A72629D5h)
00007FF6A72629C4  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A72629CB  xor          edx, edx
00007FF6A72629CD  mov          r8, r12
00007FF6A72629D0  call         HeapFree (00007FF6A735A4DCh)



// decode_shader
00007FF6A72622B8  mov          rbx, qword ptr [rsp+3A8h]
00007FF6A72622C0  mov          rsi, qword ptr [rsp+3B8h]
00007FF6A72622C8  test         rsi, rsi
00007FF6A72622CB  je           static void Fabled_Engine::main+3CCh (00007FF6A726230Ch)
00007FF6A72622CD  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A72622D4  test         rcx, rcx
00007FF6A72622D7  jne          static void Fabled_Engine::main+3B1h (00007FF6A72622F1h)
00007FF6A72622D9  call         GetProcessHeap (00007FF6A735A4E2h)
00007FF6A72622DE  test         rax, rax
00007FF6A72622E1  je           static void Fabled_Engine::main+50ABh (00007FF6A7266FEBh)
00007FF6A72622E7  mov          rcx, rax
00007FF6A72622EA  mov          qword ptr [00007FF6A73861D8h], rax
00007FF6A72622F1  xor          edx, edx
00007FF6A72622F3  mov          r8, rsi
00007FF6A72622F6  call         HeapAlloc (00007FF6A735A4E8h)
00007FF6A72622FB  test         rax, rax
00007FF6A72622FE  je           static void Fabled_Engine::main+50ABh (00007FF6A7266FEBh)
00007FF6A7262304  mov          rdi, rax
00007FF6A7262307  mov          rbp, rsi
00007FF6A726230A  jmp          static void Fabled_Engine::main+3D3h (00007FF6A7262313h)
00007FF6A726230C  mov          edi, 1h
00007FF6A7262311  xor          ebp, ebp
00007FF6A7262313  mov          rcx, rdi
00007FF6A7262316  mov          rdx, rbx
00007FF6A7262319  mov          r8, rsi
00007FF6A726231C  call         memcpy (00007FF6A735B5C7h)
00007FF6A7262321  mov          rax, qword ptr [00007FF6A737EC90h]
00007FF6A7262328  xorps        xmm0, xmm0
00007FF6A726232B  movups       xmmword ptr [rsp+1D8h], xmm0
00007FF6A7262333  mov          qword ptr [rsp+1D0h], rax
00007FF6A726233B  mov          dword ptr [rsp+3E0h], 0h
00007FF6A7262346  mov          word ptr [rsp+3E4h], 0h
00007FF6A7262350  mov          dword ptr [rsp+3D4h], 0h
00007FF6A726235B  movaps       xmmword ptr [rsp+3C0h], xmm0
00007FF6A7262363  mov          qword ptr [rsp+3D8h], 7h
00007FF6A726236F  mov          byte ptr [rsp+3E0h], 1h
00007FF6A7262377  lea          rcx, [rsp+250h]
00007FF6A726237F  lea          r9, [rsp+3C0h]
00007FF6A7262387  mov          rdx, rdi
00007FF6A726238A  mov          r8, rsi
00007FF6A726238D  call         static void std::sys::windows::fs::File::open (00007FF6A7354AC0h)
00007FF6A7262392  mov          rbx, qword ptr [rsp+250h]
00007FF6A726239A  mov          rax, qword ptr [rsp+258h]
00007FF6A72623A2  mov          qword ptr [rsp+40h], rax
00007FF6A72623A7  mov          rsi, qword ptr [rsp+260h]
00007FF6A72623AF  test         rbp, rbp
00007FF6A72623B2  je           static void Fabled_Engine::main+485h (00007FF6A72623C5h)
00007FF6A72623B4  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A72623BB  xor          edx, edx
00007FF6A72623BD  mov          r8, rdi
00007FF6A72623C0  call         HeapFree (00007FF6A735A4DCh)
00007FF6A72623C5  test         rbx, rbx
00007FF6A72623C8  jne          static void Fabled_Engine::main+4F20h (00007FF6A7266E60h)
00007FF6A72623CE  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A72623D5  test         rcx, rcx
00007FF6A72623D8  jne          static void Fabled_Engine::main+4B2h (00007FF6A72623F2h)
00007FF6A72623DA  call         GetProcessHeap (00007FF6A735A4E2h)
00007FF6A72623DF  test         rax, rax
00007FF6A72623E2  je           static void Fabled_Engine::main+4EC8h (00007FF6A7266E08h)
00007FF6A72623E8  mov          rcx, rax
00007FF6A72623EB  mov          qword ptr [00007FF6A73861D8h], rax
00007FF6A72623F2  mov          r8d, 2000h
00007FF6A72623F8  xor          edx, edx
00007FF6A72623FA  call         HeapAlloc (00007FF6A735A4E8h)
00007FF6A72623FF  mov          qword ptr [rsp+D0h], rax
00007FF6A7262407  test         rax, rax
00007FF6A726240A  je           static void Fabled_Engine::main+4EC8h (00007FF6A7266E08h)
00007FF6A7262410  mov          rbx, qword ptr [rsp+1E0h]
00007FF6A7262418  xor          r15d, r15d
00007FF6A726241B  mov          rbp, rbx
00007FF6A726241E  xor          r13d, r13d
00007FF6A7262421  mov          qword ptr [rsp+C0h], rbx
00007FF6A7262429  cmp          rbx, rbp
00007FF6A726242C  jne          static void Fabled_Engine::main+570h (00007FF6A72624B0h)
00007FF6A7262432  jmp          static void Fabled_Engine::main+515h (00007FF6A7262455h)
00007FF6A7262434  mov          rsi, r12
00007FF6A7262437  shl          rsi, 20h
00007FF6A726243B  mov          ecx, r12d
00007FF6A726243E  call         static void std::sys::windows::decode_error_kind (00007FF6A7350E10h)
00007FF6A7262443  cmp          al, Fh
00007FF6A7262445  jne          static void Fabled_Engine::main+ADCh (00007FF6A7262A1Ch)
00007FF6A726244B  nop          dword ptr [rax+rax*1], eax
00007FF6A7262450  cmp          rbx, rbp
00007FF6A7262453  jne          static void Fabled_Engine::main+570h (00007FF6A72624B0h)
00007FF6A7262455  mov          rdx, qword ptr [rsp+1D8h]
00007FF6A726245D  mov          rax, rdx
00007FF6A7262460  sub          rax, rbp
00007FF6A7262463  cmp          rax, 1Fh
00007FF6A7262467  jbe          static void Fabled_Engine::main+53Fh (00007FF6A726247Fh)
00007FF6A7262469  mov          qword ptr [rsp+1E0h], rdx
00007FF6A7262471  cmp          rdx, rbx
00007FF6A7262474  jb           static void Fabled_Engine::main+4EA1h (00007FF6A7266DE1h)
00007FF6A726247A  mov          rbp, rdx
00007FF6A726247D  jmp          static void Fabled_Engine::main+570h (00007FF6A72624B0h)
00007FF6A726247F  mov          r8d, 20h
00007FF6A7262485  lea          rcx, [rsp+1D0h]
00007FF6A726248D  mov          rdx, rbp
00007FF6A7262490  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<u8,alloc::alloc::Global> (00007FF6A735BE30h)
00007FF6A7262495  mov          rdx, qword ptr [rsp+1D8h]
00007FF6A726249D  mov          qword ptr [rsp+1E0h], rdx
00007FF6A72624A5  cmp          rdx, rbx
00007FF6A72624A8  jae          static void Fabled_Engine::main+53Ah (00007FF6A726247Ah)
00007FF6A72624AA  jmp          static void Fabled_Engine::main+4EA1h (00007FF6A7266DE1h)
00007FF6A72624AF  nop
00007FF6A72624B0  mov          rsi, rbp
00007FF6A72624B3  sub          rsi, rbx
00007FF6A72624B6  jb           static void Fabled_Engine::main+48DFh (00007FF6A726681Fh)
00007FF6A72624BC  mov          rdi, qword ptr [rsp+1D0h]
00007FF6A72624C4  lea          r14, [rdi+rbx*1]
00007FF6A72624C8  cmp          r13, r15
00007FF6A72624CB  mov          qword ptr [rsp+48h], rdi
00007FF6A72624D0  jne          static void Fabled_Engine::main+600h (00007FF6A7262540h)
00007FF6A72624D2  cmp          rsi, 1FFFh
00007FF6A72624D9  jbe          static void Fabled_Engine::main+600h (00007FF6A7262540h)
00007FF6A72624DB  mov          dword ptr [rsp+3C0h], 0h
00007FF6A72624E6  mov          eax, FFFFFFFFh
00007FF6A72624EB  cmp          rsi, rax
00007FF6A72624EE  mov          r8d, FFFFFFFFh
00007FF6A72624F4  cmovb        r8d, esi
00007FF6A72624F8  mov          qword ptr [rsp+20h], 0h
00007FF6A7262501  mov          rcx, qword ptr [rsp+40h]
00007FF6A7262506  mov          rdx, r14
00007FF6A7262509  lea          r9, [rsp+3C0h]
00007FF6A7262511  call         qword ptr [__imp_ReadFile (00007FF6A7368040h)]
00007FF6A7262517  test         eax, eax
00007FF6A7262519  je           static void Fabled_Engine::main+64Ch (00007FF6A726258Ch)
00007FF6A726251B  mov          edi, dword ptr [rsp+3C0h]
00007FF6A7262522  xor          r15d, r15d
00007FF6A7262525  xor          r13d, r13d
00007FF6A7262528  test         rdi, rdi
00007FF6A726252B  jne          static void Fabled_Engine::main+6FAh (00007FF6A726263Ah)
00007FF6A7262531  jmp          static void Fabled_Engine::main+714h (00007FF6A7262654h)
00007FF6A7262536  nop          word ptr cs:[rax+rax*1], ax
00007FF6A7262540  cmp          r13, r15
00007FF6A7262543  jb           static void Fabled_Engine::main+69Dh (00007FF6A72625DDh)
00007FF6A7262549  mov          dword ptr [rsp+3C0h], 0h
00007FF6A7262554  mov          qword ptr [rsp+20h], 0h
00007FF6A726255D  mov          rcx, qword ptr [rsp+40h]
00007FF6A7262562  mov          rdx, qword ptr [rsp+D0h]
00007FF6A726256A  mov          r8d, 2000h
00007FF6A7262570  lea          r9, [rsp+3C0h]
00007FF6A7262578  call         qword ptr [__imp_ReadFile (00007FF6A7368040h)]
00007FF6A726257E  test         eax, eax
00007FF6A7262580  je           static void Fabled_Engine::main+680h (00007FF6A72625C0h)
00007FF6A7262582  mov          r15d, dword ptr [rsp+3C0h]
00007FF6A726258A  jmp          static void Fabled_Engine::main+69Ah (00007FF6A72625DAh)
00007FF6A726258C  call         GetLastError (00007FF6A735A4EEh)
00007FF6A7262591  mov          r12d, eax
00007FF6A7262594  mov          ecx, eax
00007FF6A7262596  call         static void std::sys::windows::decode_error_kind (00007FF6A7350E10h)
00007FF6A726259B  cmp          al, 8h
00007FF6A726259D  je           static void Fabled_Engine::main+714h (00007FF6A7262654h)
00007FF6A72625A3  xor          esi, esi
00007FF6A72625A5  xor          r13d, r13d
00007FF6A72625A8  xor          r15d, r15d
00007FF6A72625AB  mov          ecx, r12d
00007FF6A72625AE  call         static void std::sys::windows::decode_error_kind (00007FF6A7350E10h)
00007FF6A72625B3  cmp          al, Fh
00007FF6A72625B5  je           static void Fabled_Engine::main+510h (00007FF6A7262450h)
00007FF6A72625BB  jmp          static void Fabled_Engine::main+ADCh (00007FF6A7262A1Ch)
00007FF6A72625C0  call         GetLastError (00007FF6A735A4EEh)
00007FF6A72625C5  mov          r12d, eax
00007FF6A72625C8  mov          ecx, eax
00007FF6A72625CA  call         static void std::sys::windows::decode_error_kind (00007FF6A7350E10h)
00007FF6A72625CF  cmp          al, 8h
00007FF6A72625D1  jne          static void Fabled_Engine::main+4F4h (00007FF6A7262434h)
00007FF6A72625D7  xor          r15d, r15d
00007FF6A72625DA  xor          r13d, r13d
00007FF6A72625DD  cmp          r15, 2001h
00007FF6A72625E4  jae          static void Fabled_Engine::main+4EB2h (00007FF6A7266DF2h)
00007FF6A72625EA  mov          rax, qword ptr [rsp+D0h]
00007FF6A72625F2  lea          rdx, [rax+r13*1]
00007FF6A72625F6  mov          rdi, r15
00007FF6A72625F9  sub          rdi, r13
00007FF6A72625FC  cmp          rdi, rsi
00007FF6A72625FF  cmovae       rdi, rsi
00007FF6A7262603  cmp          rdi, 1h
00007FF6A7262607  jne          static void Fabled_Engine::main+6E0h (00007FF6A7262620h)
00007FF6A7262609  test         rsi, rsi
00007FF6A726260C  je           static void Fabled_Engine::main+16A0h (00007FF6A72635E0h)
00007FF6A7262612  movzx        eax, byte ptr [rdx]
00007FF6A7262615  mov          byte ptr [r14], al
00007FF6A7262618  jmp          static void Fabled_Engine::main+6EBh (00007FF6A726262Bh)
00007FF6A726261A  nop          word ptr [rax+rax*1], ax
00007FF6A7262620  mov          rcx, r14
00007FF6A7262623  mov          r8, rdi
00007FF6A7262626  call         memcpy (00007FF6A735B5C7h)
00007FF6A726262B  add          r13, rdi
00007FF6A726262E  cmp          r13, r15
00007FF6A7262631  cmova        r13, r15
00007FF6A7262635  test         rdi, rdi
00007FF6A7262638  je           static void Fabled_Engine::main+714h (00007FF6A7262654h)
00007FF6A726263A  cmp          rdi, rsi
00007FF6A726263D  ja           static void Fabled_Engine::main+4908h (00007FF6A7266848h)
00007FF6A7262643  add          rbx, rdi
00007FF6A7262646  cmp          rbx, rbp
00007FF6A7262649  jne          static void Fabled_Engine::main+570h (00007FF6A72624B0h)
00007FF6A726264F  jmp          static void Fabled_Engine::main+515h (00007FF6A7262455h)
00007FF6A7262654  mov          rsi, rbx
00007FF6A7262657  mov          rbp, qword ptr [rsp+C0h]
00007FF6A726265F  sub          rsi, rbp
00007FF6A7262662  xor          edi, edi
00007FF6A7262664  mov          r8, rbx
00007FF6A7262667  sub          r8, rbp
00007FF6A726266A  jb           static void Fabled_Engine::main+B00h (00007FF6A7262A40h)
00007FF6A7262670  mov          rdx, qword ptr [rsp+48h]
00007FF6A7262675  add          rdx, rbp
00007FF6A7262678  lea          rcx, [rsp+3C0h]
00007FF6A7262680  call         static void core::str::converts::from_utf8 (00007FF6A7273320h)
00007FF6A7262685  mov          rax, qword ptr [rsp+3C0h]
00007FF6A726268D  test         rax, rax
00007FF6A7262690  cmovne       rbx, rbp
00007FF6A7262694  setne        cl
00007FF6A7262697  mov          qword ptr [rsp+1E0h], rbx
00007FF6A726269F  or           cl, dil
00007FF6A72626A2  cmp          cl, 1h
00007FF6A72626A5  je           static void Fabled_Engine::main+4F63h (00007FF6A7266EA3h)
00007FF6A72626AB  mov          rcx, qword ptr [rsp+40h]
00007FF6A72626B0  call         CloseHandle (00007FF6A735A4F4h)
00007FF6A72626B5  mov          rcx, qword ptr [00007FF6A73861D8h]
00007FF6A72626BC  xor          edx, edx
00007FF6A72626BE  mov          r8, qword ptr [rsp+D0h]
00007FF6A72626C6  call         HeapFree (00007FF6A735A4DCh)
00007FF6A72626CB  mov          r15, qword ptr [rsp+1E0h]
00007FF6A72626D3  mov          r12, qword ptr [rsp+1D0h]
00007FF6A72626DB  mov          r14, qword ptr [rsp+1D8h]