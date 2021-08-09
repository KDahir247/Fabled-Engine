00007FF69FC0135B  mov          rcx, qword ptr [00007FF69FC2A1C8h]
00007FF69FC01362  test         rcx, rcx
00007FF69FC01365  jne          static void Fabled_Engine::main+BFh (00007FF69FC0137Fh)
00007FF69FC01367  call         GetProcessHeap (00007FF69FC1F312h)
00007FF69FC0136C  test         rax, rax
00007FF69FC0136F  je           static void Fabled_Engine::main+8E7h (00007FF69FC01BA7h)
00007FF69FC01375  mov          rcx, rax
00007FF69FC01378  mov          qword ptr [00007FF69FC2A1C8h], rax
00007FF69FC0137F  mov          r8d, 5A0h
00007FF69FC01385  xor          edx, edx
00007FF69FC01387  call         HeapAlloc (00007FF69FC1F318h)
00007FF69FC0138C  test         rax, rax
00007FF69FC0138F  je           static void Fabled_Engine::main+8E7h (00007FF69FC01BA7h)
00007FF69FC01395  mov          qword ptr [rsp+28h], rax
00007FF69FC0139A  movaps       xmm0, xmmword ptr [__xmm@000000000000000000000000000000b4 (00007FF69FC22530h)]
00007FF69FC013A1  movups       xmmword ptr [rsp+30h], xmm0
00007FF69FC013A6  mov          rcx, qword ptr [00007FF69FC2A1C8h]
00007FF69FC013AD  test         rcx, rcx
00007FF69FC013B0  jne          static void Fabled_Engine::main+10Ah (00007FF69FC013CAh)
00007FF69FC013B2  call         GetProcessHeap (00007FF69FC1F312h)
00007FF69FC013B7  test         rax, rax
00007FF69FC013BA  je           static void Fabled_Engine::main+8F8h (00007FF69FC01BB8h)
00007FF69FC013C0  mov          rcx, rax
00007FF69FC013C3  mov          qword ptr [00007FF69FC2A1C8h], rax
00007FF69FC013CA  mov          r8d, 800h
00007FF69FC013D0  xor          edx, edx
00007FF69FC013D2  call         HeapAlloc (00007FF69FC1F318h)
00007FF69FC013D7  test         rax, rax
00007FF69FC013DA  je           static void Fabled_Engine::main+8F8h (00007FF69FC01BB8h)
00007FF69FC013E0  mov          qword ptr [rsp+40h], rax
00007FF69FC013E5  mov          rcx, 4000000000000000h
00007FF69FC013EF  mov          qword ptr [rax], rcx
00007FF69FC013F2  mov          dword ptr [rax+8h], 0h
00007FF69FC013F9  movaps       xmm0, xmmword ptr [__xmm@3f800000000000003f80000000000000 (00007FF69FC22540h)]
00007FF69FC01400  movups       xmmword ptr [rax+Ch], xmm0
00007FF69FC01404  movaps       xmm0, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF69FC22550h)]
00007FF69FC0140B  movups       xmmword ptr [rax+1Ch], xmm0
00007FF69FC0140F  movaps       xmm0, xmmword ptr [__xmm@3f80000000000000000000003f800000 (00007FF69FC22560h)]
00007FF69FC01416  movups       xmmword ptr [rax+2Ch], xmm0
00007FF69FC0141A  mov          dword ptr [rax+3Ch], 3F800000h
00007FF69FC01421  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000020 (00007FF69FC22570h)]
00007FF69FC01428  movups       xmmword ptr [rsp+48h], xmm0
00007FF69FC0142D  xorps        xmm0, xmm0
00007FF69FC01430  movaps       xmmword ptr [rax+40h], xmm0
00007FF69FC01434  movaps       xmm0, xmmword ptr [__xmm@00000000bf80000000000000bf800000 (00007FF69FC22580h)]
00007FF69FC0143B  movaps       xmmword ptr [rax+50h], xmm0
00007FF69FC0143F  movaps       xmm0, xmmword ptr [__xmm@3f8000000000000000000000bf800000 (00007FF69FC22590h)]
00007FF69FC01446  movaps       xmmword ptr [rax+60h], xmm0
00007FF69FC0144A  movaps       xmm0, xmmword ptr [__xmm@3f800000bf8000000000000000000000 (00007FF69FC225A0h)]
00007FF69FC01451  movaps       xmmword ptr [rax+70h], xmm0
00007FF69FC01455  mov          qword ptr [rsp+50h], 2h
00007FF69FC0145E  xor          ecx, ecx
00007FF69FC01460  movss        xmm9, dword ptr [__real@3e567750 (00007FF69FC225B0h)]
00007FF69FC01469  movss        xmm10, dword ptr [__real@3f800000 (00007FF69FC225B4h)]
00007FF69FC01472  movaps       xmm11, xmmword ptr [__xmm@3f8000003f8000000000000000000000 (00007FF69FC225C0h)]
00007FF69FC0147A  movaps       xmm12, xmmword ptr [__xmm@40a0000040a0000040a0000040a00000 (00007FF69FC225D0h)]
00007FF69FC01482  movaps       xmm13, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF69FC225E0h)]
00007FF69FC0148A  movaps       xmm14, xmmword ptr [__xmm@00000000bf8000008000000080000000 (00007FF69FC225F0h)]
00007FF69FC01492  movss        xmm15, dword ptr [__real@41f00000 (00007FF69FC22600h)]
00007FF69FC0149B  xor          eax, eax
00007FF69FC0149D  nop          dword ptr [rax], eax
00007FF69FC014A0  cmp          rcx, 1Eh
00007FF69FC014A4  adc          rax, 0h
00007FF69FC014A8  test         rcx, rcx
00007FF69FC014AB  mov          qword ptr [rsp+98h], rcx
00007FF69FC014B3  mov          qword ptr [rsp+90h], rax
00007FF69FC014BB  js           static void Fabled_Engine::main+210h (00007FF69FC014D0h)
00007FF69FC014BD  xorps        xmm8, xmm8
00007FF69FC014C1  cvtsi2ss     xmm8, rcx
00007FF69FC014C6  jmp          static void Fabled_Engine::main+226h (00007FF69FC014E6h)
00007FF69FC014C8  nop          dword ptr [rax+rax*1], eax
00007FF69FC014D0  mov          rax, rcx
00007FF69FC014D3  shr          rax, 1h
00007FF69FC014D6  and          ecx, 1h
00007FF69FC014D9  or           rcx, rax
00007FF69FC014DC  cvtsi2ss     xmm8, rcx
00007FF69FC014E1  addss        xmm8, xmm8
00007FF69FC014E6  movaps       xmm7, xmm8
00007FF69FC014EA  mulss        xmm7, xmm9
00007FF69FC014EF  movaps       xmm0, xmm7
00007FF69FC014F2  call         sinf (00007FF69FC20439h)
00007FF69FC014F7  movaps       xmm6, xmm0
00007FF69FC014FA  movaps       xmm0, xmm7
00007FF69FC014FD  call         cosf (00007FF69FC2043Fh)
00007FF69FC01502  shufps       xmm0, xmm0, 0h
00007FF69FC01506  mulps        xmm0, xmm10
00007FF69FC0150A  shufps       xmm6, xmm6, 0h
00007FF69FC0150E  mulps        xmm6, xmm11
00007FF69FC01512  addps        xmm6, xmm0
00007FF69FC01515  mulps        xmm6, xmm12
00007FF69FC01519  addps        xmm6, xmmword ptr [__xmm@00000000000000000000000000000000 (00007FF69FC22620h)]
00007FF69FC01520  movaps       xmm1, xmm13
00007FF69FC01524  subps        xmm1, xmm6
00007FF69FC01527  movaps       xmm0, xmm6
00007FF69FC0152A  mulps        xmm0, xmm6
00007FF69FC0152D  movaps       xmm2, xmm0
00007FF69FC01530  shufps       xmm2, xmm0, 55h
00007FF69FC01534  addss        xmm2, xmm0
00007FF69FC01538  movhlps      xmm0, xmm0
00007FF69FC0153B  addss        xmm0, xmm2
00007FF69FC0153F  shufps       xmm0, xmm0, 0h
00007FF69FC01543  sqrtps       xmm2, xmm0
00007FF69FC01546  movaps       xmm0, xmm6
00007FF69FC01549  divps        xmm0, xmm2
00007FF69FC0154C  movaps       xmm2, xmm0
00007FF69FC0154F  mulps        xmm2, xmm10
00007FF69FC01553  shufps       xmm2, xmm2, D2h
00007FF69FC01557  mulps        xmm0, xmm14
00007FF69FC0155B  addps        xmm0, xmm2
00007FF69FC0155E  movaps       xmm2, xmm0
00007FF69FC01561  shufps       xmm2, xmm0, 52h
00007FF69FC01565  movaps       xmm3, xmm1
00007FF69FC01568  shufps       xmm3, xmm1, D2h
00007FF69FC0156C  movaps       xmm4, xmm0
00007FF69FC0156F  shufps       xmm4, xmm0, 49h
00007FF69FC01573  mulps        xmm3, xmm2
00007FF69FC01576  mulps        xmm1, xmm4
00007FF69FC01579  subps        xmm3, xmm1
00007FF69FC0157C  movaps       xmm7, xmm3
00007FF69FC0157F  shufps       xmm7, xmm3, 52h
00007FF69FC01583  mulps        xmm3, xmm3
00007FF69FC01586  movaps       xmm1, xmm3
00007FF69FC01589  unpckhpd     xmm1, xmm3
00007FF69FC0158D  addss        xmm1, xmm3
00007FF69FC01591  shufps       xmm3, xmm3, 55h
00007FF69FC01595  addss        xmm3, xmm1
00007FF69FC01599  shufps       xmm3, xmm3, 0h
00007FF69FC0159D  sqrtps       xmm1, xmm3
00007FF69FC015A0  divps        xmm7, xmm1
00007FF69FC015A3  movaps       xmm1, xmm7
00007FF69FC015A6  shufps       xmm1, xmm7, D2h
00007FF69FC015AA  mulps        xmm1, xmm2
00007FF69FC015AD  mulps        xmm4, xmm7
00007FF69FC015B0  subps        xmm1, xmm4
00007FF69FC015B3  movd         r12d, xmm6
00007FF69FC015B8  movaps       xmm2, xmm6
00007FF69FC015BB  shufps       xmm2, xmm6, 55h
00007FF69FC015BF  movd         eax, xmm2
00007FF69FC015C3  shl          rax, 20h
00007FF69FC015C7  or           r12, rax
00007FF69FC015CA  divss        xmm8, xmm15
00007FF69FC015CF  movd         r13d, xmm7
00007FF69FC015D4  movaps       xmm2, xmm7
00007FF69FC015D7  shufps       xmm2, xmm7, 55h
00007FF69FC015DB  movd         eax, xmm2
00007FF69FC015DF  shl          rax, 20h
00007FF69FC015E3  or           r13, rax
00007FF69FC015E6  movaps       xmm2, xmm0
00007FF69FC015E9  unpckhpd     xmm2, xmm0
00007FF69FC015ED  movd         ebp, xmm2
00007FF69FC015F1  movd         eax, xmm0
00007FF69FC015F5  shufps       xmm0, xmm0, 55h
00007FF69FC015F9  movd         edi, xmm0
00007FF69FC015FD  shl          rax, 20h
00007FF69FC01601  or           rbp, rax
00007FF69FC01604  mov          rcx, 3F80000000000000h
00007FF69FC0160E  or           rdi, rcx
00007FF69FC01611  movaps       xmm0, xmm1
00007FF69FC01614  unpckhpd     xmm0, xmm1
00007FF69FC01618  movd         r14d, xmm0
00007FF69FC0161D  movd         eax, xmm1
00007FF69FC01621  shufps       xmm1, xmm1, 55h
00007FF69FC01625  movd         r15d, xmm1
00007FF69FC0162A  shl          rax, 20h
00007FF69FC0162E  or           r14, rax
00007FF69FC01631  or           r15, rcx
00007FF69FC01634  mov          rdx, qword ptr [rsp+50h]
00007FF69FC01639  cmp          rdx, qword ptr [rsp+48h]
00007FF69FC0163E  je           static void Fabled_Engine::main+3FBh (00007FF69FC016BBh)
00007FF69FC01640  punpckhqdq   xmm6, xmm6
00007FF69FC01644  movd         eax, xmm6
00007FF69FC01648  punpckhqdq   xmm7, xmm7
00007FF69FC0164C  mov          rcx, rdx
00007FF69FC0164F  shl          rcx, 6h
00007FF69FC01653  mov          rbx, qword ptr [rsp+40h]
00007FF69FC01658  mov          qword ptr [rbx+rcx*1], r12
00007FF69FC0165C  movd         esi, xmm7
00007FF69FC01660  add          rdx, 1h
00007FF69FC01664  mov          dword ptr [rbx+rcx*1+8h], eax
00007FF69FC01668  movss        dword ptr [rbx+rcx*1+Ch], xmm8
00007FF69FC0166F  mov          dword ptr [rbx+rcx*1+10h], 0h
00007FF69FC01677  mov          qword ptr [rbx+rcx*1+14h], r13
00007FF69FC0167C  mov          dword ptr [rbx+rcx*1+1Ch], esi
00007FF69FC01680  mov          qword ptr [rbx+rcx*1+28h], rdi
00007FF69FC01685  mov          qword ptr [rbx+rcx*1+20h], rbp
00007FF69FC0168A  mov          qword ptr [rbx+rcx*1+38h], r15
00007FF69FC0168F  mov          qword ptr [rbx+rcx*1+30h], r14
00007FF69FC01694  mov          qword ptr [rsp+50h], rdx
00007FF69FC01699  cmp          qword ptr [rsp+98h], 1Dh
00007FF69FC016A2  mov          rax, qword ptr [rsp+90h]
00007FF69FC016AA  ja           static void Fabled_Engine::main+40Fh (00007FF69FC016CFh)
00007FF69FC016AC  mov          rcx, rax
00007FF69FC016AF  cmp          rax, 1Fh
00007FF69FC016B3  jb           static void Fabled_Engine::main+1E0h (00007FF69FC014A0h)
00007FF69FC016B9  jmp          static void Fabled_Engine::main+40Fh (00007FF69FC016CFh)
00007FF69FC016BB  lea          rcx, [rsp+40h]
00007FF69FC016C0  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF69FC20F70h)
00007FF69FC016C5  mov          rdx, qword ptr [rsp+50h]
00007FF69FC016CA  jmp          static void Fabled_Engine::main+380h (00007FF69FC01640h)
00007FF69FC016CF  mov          esi, 3h
00007FF69FC016D4  mov          rdx, qword ptr [rsp+38h]
00007FF69FC016D9  lea          rdi, [rsp+28h]
00007FF69FC016DE  jmp          static void Fabled_Engine::main+43Bh (00007FF69FC016FBh)
00007FF69FC016E0  mov          qword ptr [rcx+rdx*8], rsi
00007FF69FC016E4  add          rdx, 1h
00007FF69FC016E8  mov          qword ptr [rsp+38h], rdx
00007FF69FC016ED  add          rsi, 1h
00007FF69FC016F1  cmp          rsi, 21h
00007FF69FC016F5  je           static void Fabled_Engine::main+572h (00007FF69FC01832h)
00007FF69FC016FB  mov          rax, qword ptr [rsp+30h]
00007FF69FC01700  cmp          rdx, rax
00007FF69FC01703  je           static void Fabled_Engine::main+4ECh (00007FF69FC017ACh)
00007FF69FC01709  mov          rcx, qword ptr [rsp+28h]
00007FF69FC0170E  mov          qword ptr [rcx+rdx*8], 0h
00007FF69FC01716  add          rdx, 1h
00007FF69FC0171A  mov          qword ptr [rsp+38h], rdx
00007FF69FC0171F  cmp          rdx, rax
00007FF69FC01722  je           static void Fabled_Engine::main+503h (00007FF69FC017C3h)
00007FF69FC01728  lea          rbx, [rsi-1h]
00007FF69FC0172C  mov          qword ptr [rcx+rdx*8], rsi
00007FF69FC01730  add          rdx, 1h
00007FF69FC01734  mov          qword ptr [rsp+38h], rdx
00007FF69FC01739  cmp          rdx, rax
00007FF69FC0173C  je           static void Fabled_Engine::main+522h (00007FF69FC017E2h)
00007FF69FC01742  mov          qword ptr [rcx+rdx*8], rbx
00007FF69FC01746  add          rdx, 1h
00007FF69FC0174A  mov          qword ptr [rsp+38h], rdx
00007FF69FC0174F  mov          rax, qword ptr [rsp+30h]
00007FF69FC01754  cmp          rdx, rax
00007FF69FC01757  je           static void Fabled_Engine::main+53Ch (00007FF69FC017FCh)
00007FF69FC0175D  mov          rcx, qword ptr [rsp+28h]
00007FF69FC01762  mov          qword ptr [rcx+rdx*8], 1h
00007FF69FC0176A  add          rdx, 1h
00007FF69FC0176E  mov          qword ptr [rsp+38h], rdx
00007FF69FC01773  cmp          rdx, rax
00007FF69FC01776  je           static void Fabled_Engine::main+553h (00007FF69FC01813h)
00007FF69FC0177C  mov          qword ptr [rcx+rdx*8], rbx
00007FF69FC01780  add          rdx, 1h
00007FF69FC01784  mov          qword ptr [rsp+38h], rdx
00007FF69FC01789  cmp          rdx, rax
00007FF69FC0178C  jne          static void Fabled_Engine::main+420h (00007FF69FC016E0h)
00007FF69FC01792  mov          rcx, rdi
00007FF69FC01795  mov          rdx, rax
00007FF69FC01798  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF69FC20EB0h)
00007FF69FC0179D  mov          rcx, qword ptr [rsp+28h]
00007FF69FC017A2  mov          rdx, qword ptr [rsp+38h]
00007FF69FC017A7  jmp          static void Fabled_Engine::main+420h (00007FF69FC016E0h)
00007FF69FC017AC  mov          rcx, rdi
00007FF69FC017AF  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF69FC20EB0h)
00007FF69FC017B4  mov          rax, qword ptr [rsp+30h]
00007FF69FC017B9  mov          rdx, qword ptr [rsp+38h]
00007FF69FC017BE  jmp          static void Fabled_Engine::main+449h (00007FF69FC01709h)
00007FF69FC017C3  mov          rcx, rdi
00007FF69FC017C6  mov          rdx, rax
00007FF69FC017C9  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF69FC20EB0h)
00007FF69FC017CE  mov          rcx, qword ptr [rsp+28h]
00007FF69FC017D3  mov          rax, qword ptr [rsp+30h]
00007FF69FC017D8  mov          rdx, qword ptr [rsp+38h]
00007FF69FC017DD  jmp          static void Fabled_Engine::main+468h (00007FF69FC01728h)
00007FF69FC017E2  mov          rcx, rdi
00007FF69FC017E5  mov          rdx, rax
00007FF69FC017E8  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF69FC20EB0h)
00007FF69FC017ED  mov          rcx, qword ptr [rsp+28h]
00007FF69FC017F2  mov          rdx, qword ptr [rsp+38h]
00007FF69FC017F7  jmp          static void Fabled_Engine::main+482h (00007FF69FC01742h)
00007FF69FC017FC  mov          rcx, rdi
00007FF69FC017FF  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF69FC20EB0h)
00007FF69FC01804  mov          rax, qword ptr [rsp+30h]
00007FF69FC01809  mov          rdx, qword ptr [rsp+38h]
00007FF69FC0180E  jmp          static void Fabled_Engine::main+49Dh (00007FF69FC0175Dh)
00007FF69FC01813  mov          rcx, rdi
00007FF69FC01816  mov          rdx, rax
00007FF69FC01819  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<usize,alloc::alloc::Global> (00007FF69FC20EB0h)
00007FF69FC0181E  mov          rcx, qword ptr [rsp+28h]
00007FF69FC01823  mov          rax, qword ptr [rsp+30h]
00007FF69FC01828  mov          rdx, qword ptr [rsp+38h]
00007FF69FC0182D  jmp          static void Fabled_Engine::main+4BCh (00007FF69FC0177Ch)
00007FF69FC01832  mov          rax, qword ptr [rsp+38h]
00007FF69FC01837  mov          qword ptr [rsp+74h], rax
00007FF69FC0183C  movups       xmm0, xmmword ptr [rsp+28h]
00007FF69FC01841  movups       xmmword ptr [rsp+64h], xmm0
00007FF69FC01846  mov          rcx, qword ptr [00007FF69FC2A1C8h]
00007FF69FC0184D  test         rcx, rcx
00007FF69FC01850  jne          static void Fabled_Engine::main+5AAh (00007FF69FC0186Ah)
00007FF69FC01852  call         GetProcessHeap (00007FF69FC1F312h)
00007FF69FC01857  test         rax, rax
00007FF69FC0185A  je           static void Fabled_Engine::main+8FFh (00007FF69FC01BBFh)
00007FF69FC01860  mov          rcx, rax
00007FF69FC01863  mov          qword ptr [00007FF69FC2A1C8h], rax
00007FF69FC0186A  mov          r8d, 40h
00007FF69FC01870  xor          edx, edx
00007FF69FC01872  call         HeapAlloc (00007FF69FC1F318h)
00007FF69FC01877  test         rax, rax
00007FF69FC0187A  je           static void Fabled_Engine::main+8FFh (00007FF69FC01BBFh)
00007FF69FC01880  mov          rcx, qword ptr [rsp+50h]
00007FF69FC01885  mov          qword ptr [rax+10h], rcx
00007FF69FC01889  movups       xmm0, xmmword ptr [rsp+40h]
00007FF69FC0188E  movups       xmmword ptr [rax], xmm0
00007FF69FC01891  movups       xmm0, xmmword ptr [rsp+60h]
00007FF69FC01896  movups       xmm1, xmmword ptr [rsp+6Ch]
00007FF69FC0189B  movups       xmmword ptr [rax+1Ch], xmm0
00007FF69FC0189F  movups       xmmword ptr [rax+28h], xmm1
00007FF69FC018A3  mov          dword ptr [rax+18h], 0h
00007FF69FC018AA  mov          qword ptr [rsp+A0h], rax
00007FF69FC018B2  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000001 (00007FF69FC22610h)]
00007FF69FC018B9  movups       xmmword ptr [rsp+A8h], xmm0
00007FF69FC018C1  lea          rax, [rsp+A0h]