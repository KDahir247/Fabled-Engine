# --------------- Cube Dissassembly -------------------
00007FF70D621309  mov          qword ptr [rsp+C0h], 0h
00007FF70D621315  mov          dword ptr [rsp+C8h], 0h
00007FF70D621320  mov          qword ptr [rsp+D0h], 0h
00007FF70D62132C  mov          dword ptr [rsp+D8h], 0h
00007FF70D621337  xorps        xmm0, xmm0
00007FF70D62133A  movaps       xmmword ptr [rsp+E0h], xmm0
00007FF70D621342  movaps       xmmword ptr [rsp+100h], xmm0
00007FF70D62134A  mov          rcx, qword ptr [00007FF70D6431C8h]
00007FF70D621351  test         rcx, rcx
00007FF70D621354  jne          static void Fabled_Engine::main+AEh (00007FF70D62136Eh)
00007FF70D621356  call         GetProcessHeap (00007FF70D638D02h)
00007FF70D62135B  test         rax, rax
00007FF70D62135E  je           static void Fabled_Engine::main+6D0h (00007FF70D621990h)
00007FF70D621364  mov          rcx, rax
00007FF70D621367  mov          qword ptr [00007FF70D6431C8h], rax
00007FF70D62136E  mov          r8d, 600h
00007FF70D621374  xor          edx, edx
00007FF70D621376  call         HeapAlloc (00007FF70D638D08h)
00007FF70D62137B  test         rax, rax
00007FF70D62137E  je           static void Fabled_Engine::main+6D0h (00007FF70D621990h)
00007FF70D621384  mov          qword ptr [rsp+40h], rax
00007FF70D621389  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000018 (00007FF70D63C520h)]
00007FF70D621390  movups       xmmword ptr [rsp+48h], xmm0
00007FF70D621395  mov          rcx, qword ptr [rsp+C0h]
00007FF70D62139D  mov          qword ptr [rsp+60h], rcx
00007FF70D6213A2  mov          ecx, dword ptr [rsp+C8h]
00007FF70D6213A9  mov          dword ptr [rsp+68h], ecx
00007FF70D6213AD  mov          rcx, qword ptr [rsp+D0h]
00007FF70D6213B5  mov          qword ptr [rsp+70h], rcx
00007FF70D6213BA  mov          ecx, dword ptr [rsp+D8h]
00007FF70D6213C1  mov          dword ptr [rsp+78h], ecx
00007FF70D6213C5  movaps       xmm0, xmmword ptr [rsp+E0h]
00007FF70D6213CD  movaps       xmmword ptr [rsp+80h], xmm0
00007FF70D6213D5  movaps       xmm0, xmmword ptr [rsp+100h]
00007FF70D6213DD  movaps       xmmword ptr [rsp+30h], xmm0
00007FF70D6213E2  xor          ecx, ecx
00007FF70D6213E4  nop          word ptr cs:[rax+rax*1], ax
00007FF70D6213EE  nop
00007FF70D6213F0  mov          edx, dword ptr [rsp+68h]
00007FF70D6213F4  mov          dword ptr [rax+rcx*1+8h], edx
00007FF70D6213F8  mov          rdx, qword ptr [rsp+60h]
00007FF70D6213FD  mov          qword ptr [rax+rcx*1], rdx
00007FF70D621401  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF70D62140A  mov          edx, dword ptr [rsp+78h]
00007FF70D62140E  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF70D621412  mov          rdx, qword ptr [rsp+70h]
00007FF70D621417  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF70D62141C  movaps       xmm0, xmmword ptr [rsp+80h]
00007FF70D621424  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF70D621429  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF70D62142E  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF70D621433  add          rcx, 40h
00007FF70D621437  cmp          rcx, 5C0h
00007FF70D62143E  jne          static void Fabled_Engine::main+130h (00007FF70D6213F0h)
00007FF70D621440  mov          edx, dword ptr [rsp+68h]
00007FF70D621444  mov          dword ptr [rax+rcx*1+8h], edx
00007FF70D621448  mov          rdx, qword ptr [rsp+60h]
00007FF70D62144D  mov          qword ptr [rax+rcx*1], rdx
00007FF70D621451  mov          qword ptr [rax+rcx*1+Ch], 0h
00007FF70D62145A  mov          edx, dword ptr [rsp+78h]
00007FF70D62145E  mov          dword ptr [rax+rcx*1+1Ch], edx
00007FF70D621462  mov          rdx, qword ptr [rsp+70h]
00007FF70D621467  mov          qword ptr [rax+rcx*1+14h], rdx
00007FF70D62146C  movaps       xmm0, xmmword ptr [rsp+80h]
00007FF70D621474  movaps       xmmword ptr [rax+rcx*1+20h], xmm0
00007FF70D621479  movaps       xmm0, xmmword ptr [rsp+30h]
00007FF70D62147E  movaps       xmmword ptr [rax+rcx*1+30h], xmm0
00007FF70D621483  mov          qword ptr [rsp+50h], 18h
00007FF70D62148C  mov          rcx, qword ptr [00007FF70D6431C8h]
00007FF70D621493  test         rcx, rcx
00007FF70D621496  jne          static void Fabled_Engine::main+1F0h (00007FF70D6214B0h)
00007FF70D621498  call         GetProcessHeap (00007FF70D638D02h)
00007FF70D62149D  test         rax, rax
00007FF70D6214A0  je           static void Fabled_Engine::main+6D7h (00007FF70D621997h)
00007FF70D6214A6  mov          rcx, rax
00007FF70D6214A9  mov          qword ptr [00007FF70D6431C8h], rax
00007FF70D6214B0  mov          r8d, 120h
00007FF70D6214B6  mov          edx, 8h
00007FF70D6214BB  call         HeapAlloc (00007FF70D638D08h)
00007FF70D6214C0  test         rax, rax
00007FF70D6214C3  je           static void Fabled_Engine::main+6D7h (00007FF70D621997h)
00007FF70D6214C9  mov          rdi, rax
00007FF70D6214CC  mov          rdx, qword ptr [rsp+50h]
00007FF70D6214D1  mov          eax, E0h
00007FF70D6214D6  add          rax, qword ptr [rsp+40h]
00007FF70D6214DB  mov          qword ptr [rsp+F8h], rdi
00007FF70D6214E3  add          rdi, 28h
00007FF70D6214E7  xor          ecx, ecx
00007FF70D6214E9  lea          r10, [00007FF70D63DA40h]
00007FF70D6214F0  xorps        xmm0, xmm0
00007FF70D6214F3  mov          r11, rdx
00007FF70D6214F6  nop          word ptr cs:[rax+rax*1], ax
00007FF70D621500  movaps       xmm1, xmmword ptr [r10+rcx*4]
00007FF70D621505  movaps       xmm2, xmmword ptr [r10+rcx*4+60h]
00007FF70D62150B  movaps       xmm3, xmmword ptr [r10+rcx*4+C0h]
00007FF70D621514  movaps       xmm4, xmm1
00007FF70D621517  subps        xmm4, xmm3
00007FF70D62151A  movaps       xmm5, xmm2
00007FF70D62151D  addps        xmm5, xmm4
00007FF70D621520  subps        xmm4, xmm2
00007FF70D621523  addps        xmm3, xmm1
00007FF70D621526  movaps       xmm6, xmm2
00007FF70D621529  addps        xmm6, xmm3
00007FF70D62152C  subps        xmm3, xmm2
00007FF70D62152F  movaps       xmmword ptr [rsp+80h], xmm4
00007FF70D621537  movaps       xmmword ptr [rsp+90h], xmm5
00007FF70D62153F  movaps       xmmword ptr [rsp+A0h], xmm6
00007FF70D621547  movaps       xmmword ptr [rsp+B0h], xmm3
00007FF70D62154F  lea          rbp, [rcx+2h]
00007FF70D621553  lea          rsi, [rcx+1h]
00007FF70D621557  lea          rbx, [rcx+3h]
00007FF70D62155B  mov          qword ptr [rdi-28h], rbp
00007FF70D62155F  mov          qword ptr [rdi-20h], rsi
00007FF70D621563  mov          qword ptr [rdi-18h], rcx
00007FF70D621567  mov          qword ptr [rdi-10h], rcx
00007FF70D62156B  mov          qword ptr [rdi-8h], rbx
00007FF70D62156F  mov          qword ptr [rdi], rbp
00007FF70D621572  cmp          rdx, rcx
00007FF70D621575  jb           static void Fabled_Engine::main+6A8h (00007FF70D621968h)
00007FF70D62157B  cmp          r11, 3h
00007FF70D62157F  jbe          static void Fabled_Engine::main+6B6h (00007FF70D621976h)
00007FF70D621585  movd         ebp, xmm1
00007FF70D621589  movaps       xmm2, xmm1
00007FF70D62158C  shufps       xmm2, xmm1, 55h
00007FF70D621590  movd         esi, xmm2
00007FF70D621594  shl          rsi, 20h
00007FF70D621598  or           rbp, rsi
00007FF70D62159B  mov          esi, dword ptr [rsp+88h]
00007FF70D6215A2  mov          rbx, qword ptr [rsp+80h]
00007FF70D6215AA  mov          r15d, dword ptr [rsp+98h]
00007FF70D6215B2  mov          r12, qword ptr [rsp+90h]
00007FF70D6215BA  mov          r13, qword ptr [rsp+A0h]
00007FF70D6215C2  mov          r14d, dword ptr [rsp+A8h]
00007FF70D6215CA  mov          r8d, dword ptr [rsp+B8h]
00007FF70D6215D2  mov          r9, qword ptr [rsp+B0h]
00007FF70D6215DA  mov          qword ptr [rax-E0h], rbx
00007FF70D6215E1  punpckhqdq   xmm1, xmm1
00007FF70D6215E5  mov          dword ptr [rax-D8h], esi
00007FF70D6215EB  movd         esi, xmm1
00007FF70D6215EF  mov          qword ptr [rax-D4h], 3F800000h
00007FF70D6215FA  mov          qword ptr [rax-CCh], rbp
00007FF70D621601  mov          dword ptr [rax-C4h], esi
00007FF70D621607  movaps       xmmword ptr [rax-C0h], xmm0
00007FF70D62160E  movaps       xmmword ptr [rax-B0h], xmm0
00007FF70D621615  mov          qword ptr [rax-A0h], r12
00007FF70D62161C  mov          dword ptr [rax-98h], r15d
00007FF70D621623  mov          rbx, 3F8000003F800000h
00007FF70D62162D  mov          qword ptr [rax-94h], rbx
00007FF70D621634  mov          qword ptr [rax-8Ch], rbp
00007FF70D62163B  mov          dword ptr [rax-84h], esi
00007FF70D621641  movaps       xmmword ptr [rax-80h], xmm0
00007FF70D621645  movaps       xmmword ptr [rax-70h], xmm0
00007FF70D621649  mov          qword ptr [rax-60h], r13
00007FF70D62164D  mov          dword ptr [rax-58h], r14d
00007FF70D621651  mov          rbx, 3F80000000000000h
00007FF70D62165B  mov          qword ptr [rax-54h], rbx
00007FF70D62165F  mov          qword ptr [rax-4Ch], rbp
00007FF70D621663  mov          dword ptr [rax-44h], esi
00007FF70D621666  movaps       xmmword ptr [rax-40h], xmm0
00007FF70D62166A  movaps       xmmword ptr [rax-30h], xmm0
00007FF70D62166E  mov          qword ptr [rax-20h], r9
00007FF70D621672  mov          dword ptr [rax-18h], r8d
00007FF70D621676  mov          qword ptr [rax-14h], 0h
00007FF70D62167E  mov          qword ptr [rax-Ch], rbp
00007FF70D621682  mov          dword ptr [rax-4h], esi
00007FF70D621685  movaps       xmmword ptr [rax+10h], xmm0
00007FF70D621689  movaps       xmmword ptr [rax], xmm0
00007FF70D62168C  add          rcx, 4h
00007FF70D621690  add          rax, 100h
00007FF70D621696  add          r11, FFFFFFFFFFFFFFFCh
00007FF70D62169A  add          rdi, 30h
00007FF70D62169E  cmp          rcx, 18h
00007FF70D6216A2  jne          static void Fabled_Engine::main+240h (00007FF70D621500h)
00007FF70D6216A8  mov          rcx, qword ptr [00007FF70D6431C8h]
00007FF70D6216AF  test         rcx, rcx
00007FF70D6216B2  jne          static void Fabled_Engine::main+40Ch (00007FF70D6216CCh)
00007FF70D6216B4  call         GetProcessHeap (00007FF70D638D02h)
00007FF70D6216B9  test         rax, rax
00007FF70D6216BC  je           static void Fabled_Engine::main+6E8h (00007FF70D6219A8h)
00007FF70D6216C2  mov          rcx, rax
00007FF70D6216C5  mov          qword ptr [00007FF70D6431C8h], rax
00007FF70D6216CC  mov          r8d, 40h
00007FF70D6216D2  xor          edx, edx
00007FF70D6216D4  call         HeapAlloc (00007FF70D638D08h)
00007FF70D6216D9  test         rax, rax
00007FF70D6216DC  je           static void Fabled_Engine::main+6E8h (00007FF70D6219A8h)
00007FF70D6216E2  mov          rsi, rax
00007FF70D6216E5  mov          rax, qword ptr [rsp+50h]
00007FF70D6216EA  mov          qword ptr [rsi+10h], rax
00007FF70D6216EE  movups       xmm0, xmmword ptr [rsp+40h]
00007FF70D6216F3  movups       xmmword ptr [rsi], xmm0
00007FF70D6216F6  mov          rax, qword ptr [rsp+F8h]
00007FF70D6216FE  mov          qword ptr [rsi+20h], rax
00007FF70D621702  movaps       xmm0, xmmword ptr [__xmm@00000000000000240000000000000024 (00007FF70D63C530h)]
00007FF70D621709  movups       xmmword ptr [rsi+28h], xmm0
00007FF70D62170D  mov          dword ptr [rsi+18h], 0h
00007FF70D621714  lea          rax, [00007FF70D63FC08h]
00007FF70D62171B  mov          qword ptr [rsp+E0h], rax
00007FF70D621723  mov          qword ptr [rsp+E8h], 6hr [rsp+E8h], 6h