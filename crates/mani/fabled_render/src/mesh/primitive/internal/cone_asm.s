# --------------- Cone Dissassembly -------------------
00007FF6D9C51527  mov          rcx, qword ptr [00007FF6D9C7A250h]
00007FF6D9C5152E  test         rcx, rcx
00007FF6D9C51531  jne          static void Fabled_Engine::main+BBh (00007FF6D9C5154Bh)
00007FF6D9C51533  call         GetProcessHeap (00007FF6D9C6F532h)
00007FF6D9C51538  test         rax, rax
00007FF6D9C5153B  je           static void Fabled_Engine::main+83Ah (00007FF6D9C51CCAh)
00007FF6D9C51541  mov          rcx, rax
00007FF6D9C51544  mov          qword ptr [00007FF6D9C7A250h], rax
00007FF6D9C5154B  mov          r8d, C00h
00007FF6D9C51551  mov          edx, 8h
00007FF6D9C51556  call         HeapAlloc (00007FF6D9C6F538h)
00007FF6D9C5155B  test         rax, rax
00007FF6D9C5155E  je           static void Fabled_Engine::main+83Ah (00007FF6D9C51CCAh)
00007FF6D9C51564  mov          r15, rax
00007FF6D9C51567  mov          rcx, qword ptr [00007FF6D9C7A250h]
00007FF6D9C5156E  test         rcx, rcx
00007FF6D9C51571  jne          static void Fabled_Engine::main+FBh (00007FF6D9C5158Bh)
00007FF6D9C51573  call         GetProcessHeap (00007FF6D9C6F532h)
00007FF6D9C51578  test         rax, rax
00007FF6D9C5157B  je           static void Fabled_Engine::main+841h (00007FF6D9C51CD1h)
00007FF6D9C51581  mov          rcx, rax
00007FF6D9C51584  mov          qword ptr [00007FF6D9C7A250h], rax
00007FF6D9C5158B  mov          r8d, 1100h
00007FF6D9C51591  xor          edx, edx
00007FF6D9C51593  call         HeapAlloc (00007FF6D9C6F538h)
00007FF6D9C51598  test         rax, rax
00007FF6D9C5159B  je           static void Fabled_Engine::main+841h (00007FF6D9C51CD1h)
00007FF6D9C515A1  mov          rbx, rax
00007FF6D9C515A4  mov          qword ptr [rsp+30h], rax
00007FF6D9C515A9  mov          qword ptr [rsp+40h], 0h
00007FF6D9C515B2  movaps       xmm0, xmmword ptr [__xmm@3f8000000000000000000000bf800000 (00007FF6D9C72510h)]
00007FF6D9C515B9  movaps       xmmword ptr [rax], xmm0
00007FF6D9C515BC  movaps       xmm1, xmmword ptr [__xmm@3f8000003f8000000000000000000000 (00007FF6D9C72520h)]
00007FF6D9C515C3  movaps       xmmword ptr [rax+10h], xmm1
00007FF6D9C515C7  movaps       xmm9, xmmword ptr [__xmm@00000000000000003f80000000000000 (00007FF6D9C72530h)]
00007FF6D9C515CF  movaps       xmmword ptr [rax+20h], xmm9
00007FF6D9C515D4  movaps       xmm1, xmmword ptr [__xmm@000000003f800000000000003f800000 (00007FF6D9C72540h)]
00007FF6D9C515DB  movaps       xmmword ptr [rax+30h], xmm1
00007FF6D9C515DF  mov          qword ptr [rsp+40h], 1h
00007FF6D9C515E8  movaps       xmmword ptr [rax+40h], xmm0
00007FF6D9C515EC  movaps       xmm0, xmmword ptr [__xmm@3f800000bf8000000000000000000000 (00007FF6D9C72550h)]
00007FF6D9C515F3  movaps       xmmword ptr [rax+50h], xmm0
00007FF6D9C515F7  movaps       xmm10, xmmword ptr [__xmm@0000000000000000bf80000000000000 (00007FF6D9C72560h)]
00007FF6D9C515FF  movaps       xmmword ptr [rax+60h], xmm10
00007FF6D9C51604  movaps       xmm0, xmmword ptr [__xmm@00000000bf80000000000000bf800000 (00007FF6D9C72570h)]
00007FF6D9C5160B  movaps       xmmword ptr [rax+70h], xmm0
00007FF6D9C5160F  movaps       xmm0, xmmword ptr [__xmm@00000000000000020000000000000044 (00007FF6D9C72580h)]
00007FF6D9C51616  movups       xmmword ptr [rsp+38h], xmm0
00007FF6D9C5161B  mov          edi, 2h
00007FF6D9C51620  mov          r12d, 44h
00007FF6D9C51626  xor          esi, esi
00007FF6D9C51628  movss        xmm11, dword ptr [__real@3dc90fdb (00007FF6D9C72590h)]
00007FF6D9C51631  movss        xmm12, dword ptr [__real@3f800000 (00007FF6D9C72594h)]
00007FF6D9C5163A  movaps       xmm13, xmmword ptr [__xmm@80000000800000008000000080000000 (00007FF6D9C725A0h)]
00007FF6D9C51642  xorps        xmm14, xmm14
00007FF6D9C51646  movss        xmm15, dword ptr [__real@3c800000 (00007FF6D9C725B0h)]
00007FF6D9C5164F  lea          r14, [rsp+30h]
00007FF6D9C51654  test         rsi, rsi
00007FF6D9C51657  js           static void Fabled_Engine::main+1E0h (00007FF6D9C51670h)
00007FF6D9C51659  nop          dword ptr [rax], eax
00007FF6D9C51660  xorps        xmm8, xmm8
00007FF6D9C51664  cvtsi2ss     xmm8, rsi
00007FF6D9C51669  jmp          static void Fabled_Engine::main+1FCh (00007FF6D9C5168Ch)
00007FF6D9C5166B  nop          dword ptr [rax+rax*1], eax
00007FF6D9C51670  mov          rax, rsi
00007FF6D9C51673  shr          rax, 1h
00007FF6D9C51676  mov          ecx, esi
00007FF6D9C51678  and          ecx, 1h
00007FF6D9C5167B  or           rcx, rax
00007FF6D9C5167E  xorps        xmm8, xmm8
00007FF6D9C51682  cvtsi2ss     xmm8, rcx
00007FF6D9C51687  addss        xmm8, xmm8
00007FF6D9C5168C  movaps       xmm7, xmm8
00007FF6D9C51690  mulss        xmm7, xmm11
00007FF6D9C51695  movaps       xmm0, xmm7
00007FF6D9C51698  call         sinf (00007FF6D9C70649h)
00007FF6D9C5169D  movaps       xmm6, xmm0
00007FF6D9C516A0  movaps       xmm0, xmm7
00007FF6D9C516A3  call         cosf (00007FF6D9C7064Fh)
00007FF6D9C516A8  shufps       xmm0, xmm0, 0h
00007FF6D9C516AC  mulps        xmm0, xmm12
00007FF6D9C516B0  shufps       xmm6, xmm6, 0h
00007FF6D9C516B4  mulps        xmm6, xmmword ptr [__xmm@3f8000003f8000000000000000000000 (00007FF6D9C72520h)]
00007FF6D9C516BB  addps        xmm6, xmm0
00007FF6D9C516BE  addps        xmm6, xmm10
00007FF6D9C516C2  movaps       xmm0, xmm9
00007FF6D9C516C6  subps        xmm0, xmm6
00007FF6D9C516C9  movaps       xmm1, xmm6
00007FF6D9C516CC  mulps        xmm1, xmm6
00007FF6D9C516CF  movaps       xmm2, xmm1
00007FF6D9C516D2  shufps       xmm2, xmm1, 55h
00007FF6D9C516D6  addss        xmm2, xmm1
00007FF6D9C516DA  movhlps      xmm1, xmm1
00007FF6D9C516DD  addss        xmm1, xmm2
00007FF6D9C516E1  shufps       xmm1, xmm1, 0h
00007FF6D9C516E5  sqrtps       xmm1, xmm1
00007FF6D9C516E8  movaps       xmm2, xmm6
00007FF6D9C516EB  divps        xmm2, xmm1
00007FF6D9C516EE  movq         xmm1, xmm2
00007FF6D9C516F2  movhlps      xmm2, xmm2
00007FF6D9C516F5  xorps        xmm2, xmm13
00007FF6D9C516F9  xorps        xmm3, xmm3
00007FF6D9C516FC  movss        xmm3, xmm2
00007FF6D9C51700  shufps       xmm3, xmm1, 84h
00007FF6D9C51704  movaps       xmm1, xmm0
00007FF6D9C51707  shufps       xmm1, xmm0, D2h
00007FF6D9C5170B  mulps        xmm1, xmm3
00007FF6D9C5170E  shufps       xmm3, xmm3, D2h
00007FF6D9C51712  mulps        xmm3, xmm0
00007FF6D9C51715  subps        xmm1, xmm3
00007FF6D9C51718  movaps       xmm7, xmm1
00007FF6D9C5171B  shufps       xmm7, xmm1, D2h
00007FF6D9C5171F  mulps        xmm1, xmm1
00007FF6D9C51722  movaps       xmm0, xmm1
00007FF6D9C51725  unpckhpd     xmm0, xmm1
00007FF6D9C51729  addss        xmm0, xmm1
00007FF6D9C5172D  shufps       xmm1, xmm1, 55h
00007FF6D9C51731  addss        xmm1, xmm0
00007FF6D9C51735  shufps       xmm1, xmm1, 0h
00007FF6D9C51739  sqrtps       xmm0, xmm1
00007FF6D9C5173C  divps        xmm7, xmm0
00007FF6D9C5173F  mulss        xmm8, xmm15
00007FF6D9C51744  cmp          rdi, r12
00007FF6D9C51747  jne          static void Fabled_Engine::main+2CEh (00007FF6D9C5175Eh)
00007FF6D9C51749  mov          rcx, r14
00007FF6D9C5174C  mov          rdx, r12
00007FF6D9C5174F  call         static void alloc::raw_vec::RawVec<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global>::reserve_for_push<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF6D9C5B700h)
00007FF6D9C51754  mov          rbx, qword ptr [rsp+30h]
00007FF6D9C51759  mov          rdi, qword ptr [rsp+40h]
00007FF6D9C5175E  movaps       xmm0, xmm6
00007FF6D9C51761  shufps       xmm0, xmm6, 55h
00007FF6D9C51765  movaps       xmm1, xmm7
00007FF6D9C51768  shufps       xmm1, xmm7, 55h
00007FF6D9C5176C  movaps       xmm2, xmm6
00007FF6D9C5176F  unpckhpd     xmm2, xmm6
00007FF6D9C51773  movaps       xmm3, xmm7
00007FF6D9C51776  unpckhpd     xmm3, xmm7
00007FF6D9C5177A  shl          rdi, 6h
00007FF6D9C5177E  movaps       xmmword ptr [rbx+rdi*1+10h], xmm14
00007FF6D9C51784  movaps       xmmword ptr [rbx+rdi*1], xmm14
00007FF6D9C51789  movss        dword ptr [rbx+rdi*1+20h], xmm6
00007FF6D9C5178F  movss        dword ptr [rbx+rdi*1+24h], xmm0
00007FF6D9C51795  movss        dword ptr [rbx+rdi*1+28h], xmm2
00007FF6D9C5179B  movss        dword ptr [rbx+rdi*1+2Ch], xmm8
00007FF6D9C517A2  mov          dword ptr [rbx+rdi*1+30h], 0h
00007FF6D9C517AA  movss        dword ptr [rbx+rdi*1+34h], xmm7
00007FF6D9C517B0  movss        dword ptr [rbx+rdi*1+38h], xmm1
00007FF6D9C517B6  movss        dword ptr [rbx+rdi*1+3Ch], xmm3
00007FF6D9C517BC  mov          rdi, qword ptr [rsp+40h]
00007FF6D9C517C1  add          rdi, 1h
00007FF6D9C517C5  mov          qword ptr [rsp+40h], rdi
00007FF6D9C517CA  cmp          rsi, 40h
00007FF6D9C517CE  je           static void Fabled_Engine::main+357h (00007FF6D9C517E7h)
00007FF6D9C517D0  mov          r12, qword ptr [rsp+38h]
00007FF6D9C517D5  add          rsi, 1h
00007FF6D9C517D9  test         rsi, rsi
00007FF6D9C517DC  js           static void Fabled_Engine::main+1E0h (00007FF6D9C51670h)
00007FF6D9C517E2  jmp          static void Fabled_Engine::main+1D0h (00007FF6D9C51660h)
00007FF6D9C517E7  lea          rax, [r15+58h]
00007FF6D9C517EB  mov          ecx, 4h
00007FF6D9C517F0  lea          rdx, [rcx-2h]
00007FF6D9C517F4  lea          rbp, [rcx-1h]
00007FF6D9C517F8  mov          qword ptr [rax-58h], 0h
00007FF6D9C51800  mov          qword ptr [rax-50h], rbp
00007FF6D9C51804  mov          qword ptr [rax-48h], rdx
00007FF6D9C51808  mov          qword ptr [rax-40h], 1h
00007FF6D9C51810  mov          qword ptr [rax-38h], rdx
00007FF6D9C51814  mov          qword ptr [rax-30h], rbp
00007FF6D9C51818  mov          qword ptr [rax-28h], 0h
00007FF6D9C51820  mov          qword ptr [rax-20h], rcx
00007FF6D9C51824  mov          qword ptr [rax-18h], rbp
00007FF6D9C51828  mov          qword ptr [rax-10h], 1h
00007FF6D9C51830  mov          qword ptr [rax-8h], rbp
00007FF6D9C51834  mov          qword ptr [rax], rcx
00007FF6D9C51837  add          rcx, 2h
00007FF6D9C5183B  add          rax, 60h
00007FF6D9C5183F  cmp          rcx, 44h
00007FF6D9C51843  jne          static void Fabled_Engine::main+360h (00007FF6D9C517F0h)
00007FF6D9C51845  mov          rax, qword ptr [rsp+40h]
00007FF6D9C5184A  mov          qword ptr [rsp+70h], rax
00007FF6D9C5184F  movdqu       xmm0, xmmword ptr [rsp+30h]
00007FF6D9C51855  movdqa       xmmword ptr [rsp+60h], xmm0
00007FF6D9C5185B  mov          rcx, qword ptr [00007FF6D9C7A250h]
00007FF6D9C51862  test         rcx, rcx
00007FF6D9C51865  jne          static void Fabled_Engine::main+3EFh (00007FF6D9C5187Fh)
00007FF6D9C51867  call         GetProcessHeap (00007FF6D9C6F532h)
00007FF6D9C5186C  test         rax, rax
00007FF6D9C5186F  je           static void Fabled_Engine::main+852h (00007FF6D9C51CE2h)
00007FF6D9C51875  mov          rcx, rax
00007FF6D9C51878  mov          qword ptr [00007FF6D9C7A250h], rax
00007FF6D9C5187F  mov          r8d, 600h
00007FF6D9C51885  xor          edx, edx
00007FF6D9C51887  call         HeapAlloc (00007FF6D9C6F538h)
00007FF6D9C5188C  test         rax, rax
00007FF6D9C5188F  je           static void Fabled_Engine::main+852h (00007FF6D9C51CE2h)
00007FF6D9C51895  mov          rdi, rax
00007FF6D9C51898  lea          rax, [r15+C00h]
00007FF6D9C5189F  cmp          rdi, rax
00007FF6D9C518A2  jae          static void Fabled_Engine::main+448h (00007FF6D9C518D8h)
00007FF6D9C518A4  mov          rax, rdi
00007FF6D9C518A7  add          rax, 600h
00007FF6D9C518AD  cmp          r15, rax
00007FF6D9C518B0  jae          static void Fabled_Engine::main+448h (00007FF6D9C518D8h)
00007FF6D9C518B2  xor          eax, eax
00007FF6D9C518B4  mov          rcx, rdi
00007FF6D9C518B7  nop          word ptr [rax+rax*1], ax
00007FF6D9C518C0  mov          edx, dword ptr [r15+rax*1]
00007FF6D9C518C4  mov          dword ptr [rcx], edx
00007FF6D9C518C6  add          rcx, 4h
00007FF6D9C518CA  add          rax, 8h
00007FF6D9C518CE  cmp          rax, C00h
00007FF6D9C518D4  jne          static void Fabled_Engine::main+430h (00007FF6D9C518C0h)
00007FF6D9C518D6  jmp          static void Fabled_Engine::main+49Fh (00007FF6D9C5192Fh)
00007FF6D9C518D8  mov          eax, 6h
00007FF6D9C518DD  nop          dword ptr [rax], eax
00007FF6D9C518E0  movdqu       xmm0, xmmword ptr [r15+rax*8-30h]
00007FF6D9C518E7  movdqu       xmm1, xmmword ptr [r15+rax*8-20h]
00007FF6D9C518EE  pshufd       xmm0, xmm0, E8h
00007FF6D9C518F3  pshufd       xmm1, xmm1, E8h
00007FF6D9C518F8  punpcklqdq   xmm0, xmm1
00007FF6D9C518FC  movdqu       xmmword ptr [rdi+rax*4-18h], xmm0
00007FF6D9C51902  movdqu       xmm0, xmmword ptr [r15+rax*8-10h]
00007FF6D9C51909  movdqu       xmm1, xmmword ptr [r15+rax*8]
00007FF6D9C5190F  pshufd       xmm0, xmm0, E8h
00007FF6D9C51914  pshufd       xmm1, xmm1, E8h
00007FF6D9C51919  punpcklqdq   xmm0, xmm1
00007FF6D9C5191D  movdqu       xmmword ptr [rdi+rax*4-8h], xmm0
00007FF6D9C51923  add          rax, 8h
00007FF6D9C51927  cmp          rax, 186h
00007FF6D9C5192D  jne          static void Fabled_Engine::main+450h (00007FF6D9C518E0h)
00007FF6D9C5192F  mov          rcx, qword ptr [00007FF6D9C7A250h]
00007FF6D9C51936  xor          edx, edx
00007FF6D9C51938  mov          r8, r15
00007FF6D9C5193B  call         HeapFree (00007FF6D9C6F53Eh)
00007FF6D9C51940  mov          rcx, qword ptr [00007FF6D9C7A250h]
00007FF6D9C51947  test         rcx, rcx
00007FF6D9C5194A  jne          static void Fabled_Engine::main+4D4h (00007FF6D9C51964h)
00007FF6D9C5194C  call         GetProcessHeap (00007FF6D9C6F532h)
00007FF6D9C51951  test         rax, rax
00007FF6D9C51954  je           static void Fabled_Engine::main+863h (00007FF6D9C51CF3h)
00007FF6D9C5195A  mov          rcx, rax
00007FF6D9C5195D  mov          qword ptr [00007FF6D9C7A250h], rax
00007FF6D9C51964  mov          r8d, 40h
00007FF6D9C5196A  xor          edx, edx
00007FF6D9C5196C  call         HeapAlloc (00007FF6D9C6F538h)
00007FF6D9C51971  test         rax, rax
00007FF6D9C51974  je           static void Fabled_Engine::main+863h (00007FF6D9C51CF3h)
00007FF6D9C5197A  mov          rcx, qword ptr [rsp+70h]
00007FF6D9C5197F  mov          qword ptr [rax+10h], rcx
00007FF6D9C51983  movaps       xmm0, xmmword ptr [rsp+60h]
00007FF6D9C51988  movups       xmmword ptr [rax], xmm0
00007FF6D9C5198B  mov          qword ptr [rax+18h], 1h
00007FF6D9C51993  mov          qword ptr [rax+20h], rdi
00007FF6D9C51997  movaps       xmm0, xmmword ptr [__xmm@00000000000001800000000000000180 (00007FF6D9C725C0h)]
00007FF6D9C5199E  movups       xmmword ptr [rax+28h], xmm0
00007FF6D9C519A2  mov          qword ptr [rax+38h], 0h
00007FF6D9C519AA  mov          qword ptr [rsp+90h], rax
00007FF6D9C519B2  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000001 (00007FF6D9C72500h)]
00007FF6D9C519B9  movups       xmmword ptr [rsp+98h], xmm0
00007FF6D9C519C1  lea          rax, [rsp+90h]