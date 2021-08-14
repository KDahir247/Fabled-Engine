# --------------- Quad Dissassembly -------------------
00007FF667E412DF  movaps       xmm0, xmmword ptr [__xmm@bf00000000000000bf0000003f000000 (00007FF667E5C510h)]
00007FF667E412E6  movaps       xmmword ptr [rsp+30h], xmm0
00007FF667E412EB  movaps       xmm0, xmmword ptr [__xmm@bf000000bf000000000000003f000000 (00007FF667E5C520h)]
00007FF667E412F2  movaps       xmmword ptr [rsp+40h], xmm0
00007FF667E412F7  movaps       xmm0, xmmword ptr [__xmm@000000003f0000003f00000000000000 (00007FF667E5C530h)]
00007FF667E412FE  movaps       xmmword ptr [rsp+50h], xmm0
00007FF667E41303  movss        xmm1, dword ptr [rsp+38h]
00007FF667E41309  mov          rax, C3FA000043FA0000h
00007FF667E41313  mov          qword ptr [rsp+30h], rax
00007FF667E41318  movss        xmm0, dword ptr [__real@447a0000 (00007FF667E5C540h)]
00007FF667E41320  mov          rax, BF0000003F000000h
00007FF667E4132A  mov          qword ptr [rsp+70h], rax
00007FF667E4132F  movss        dword ptr [rsp+78h], xmm1
00007FF667E41335  mulss        xmm1, xmm0
00007FF667E41339  movss        dword ptr [rsp+38h], xmm1
00007FF667E4133F  mov          qword ptr [rsp+7Ch], 3F800000h
00007FF667E41348  mov          eax, dword ptr [00007FF667E5DAC8h]
00007FF667E4134E  mov          dword ptr [rsp+8Ch], eax
00007FF667E41355  mov          rcx, qword ptr [00007FF667E5DAC0h]
00007FF667E4135C  mov          qword ptr [rsp+84h], rcx
00007FF667E41364  movaps       xmm1, xmmword ptr [00007FF667E5DA80h]
00007FF667E4136B  movaps       xmmword ptr [rsp+90h], xmm1
00007FF667E41373  movaps       xmm2, xmmword ptr [00007FF667E5DAD0h]
00007FF667E4137A  movaps       xmmword ptr [rsp+A0h], xmm2
00007FF667E41382  movss        xmm4, dword ptr [rsp+3Ch]
00007FF667E41388  movss        xmm5, dword ptr [rsp+40h]
00007FF667E4138E  movss        xmm3, dword ptr [rsp+44h]
00007FF667E41394  movaps       xmm6, xmm4
00007FF667E41397  mulss        xmm6, xmm0
00007FF667E4139B  movss        dword ptr [rsp+3Ch], xmm6
00007FF667E413A1  movaps       xmm6, xmm5
00007FF667E413A4  mulss        xmm6, xmm0
00007FF667E413A8  movss        dword ptr [rsp+40h], xmm6
00007FF667E413AE  movss        dword ptr [rsp+B0h], xmm4
00007FF667E413B7  movss        dword ptr [rsp+B4h], xmm5
00007FF667E413C0  movss        dword ptr [rsp+B8h], xmm3
00007FF667E413C9  mulss        xmm3, xmm0
00007FF667E413CD  movss        dword ptr [rsp+44h], xmm3
00007FF667E413D3  movss        xmm3, dword ptr [__real@3f000000 (00007FF667E5C544h)]
00007FF667E413DB  addss        xmm4, xmm3
00007FF667E413DF  addss        xmm5, xmm3
00007FF667E413E3  movss        dword ptr [rsp+BCh], xmm4
00007FF667E413EC  movss        dword ptr [rsp+C0h], xmm5
00007FF667E413F5  mov          dword ptr [rsp+CCh], eax
00007FF667E413FC  mov          qword ptr [rsp+C4h], rcx
00007FF667E41404  movaps       xmmword ptr [rsp+D0h], xmm1
00007FF667E4140C  movaps       xmmword ptr [rsp+E0h], xmm2
00007FF667E41414  movss        xmm4, dword ptr [rsp+48h]
00007FF667E4141A  movss        xmm5, dword ptr [rsp+4Ch]
00007FF667E41420  movss        xmm6, dword ptr [rsp+50h]
00007FF667E41426  movaps       xmm7, xmm4
00007FF667E41429  mulss        xmm7, xmm0
00007FF667E4142D  movss        dword ptr [rsp+48h], xmm7
00007FF667E41433  movaps       xmm7, xmm5
00007FF667E41436  mulss        xmm7, xmm0
00007FF667E4143A  movss        dword ptr [rsp+4Ch], xmm7
00007FF667E41440  movss        dword ptr [rsp+F0h], xmm4
00007FF667E41449  movss        dword ptr [rsp+F4h], xmm5
00007FF667E41452  movss        dword ptr [rsp+F8h], xmm6
00007FF667E4145B  mulss        xmm6, xmm0
00007FF667E4145F  movss        dword ptr [rsp+50h], xmm6
00007FF667E41465  addss        xmm4, xmm3
00007FF667E41469  addss        xmm5, xmm3
00007FF667E4146D  movss        dword ptr [rsp+FCh], xmm4
00007FF667E41476  movss        dword ptr [rsp+100h], xmm5
00007FF667E4147F  mov          dword ptr [rsp+10Ch], eax
00007FF667E41486  mov          qword ptr [rsp+104h], rcx
00007FF667E4148E  movaps       xmmword ptr [rsp+110h], xmm1
00007FF667E41496  movaps       xmmword ptr [rsp+120h], xmm2
00007FF667E4149E  movss        xmm4, dword ptr [rsp+54h]
00007FF667E414A4  movss        xmm5, dword ptr [rsp+58h]
00007FF667E414AA  movss        xmm6, dword ptr [rsp+5Ch]
00007FF667E414B0  movaps       xmm7, xmm5
00007FF667E414B3  addss        xmm7, xmm3
00007FF667E414B7  addss        xmm3, xmm4
00007FF667E414BB  movss        dword ptr [rsp+130h], xmm4
00007FF667E414C4  mulss        xmm4, xmm0
00007FF667E414C8  movss        dword ptr [rsp+54h], xmm4
00007FF667E414CE  movss        dword ptr [rsp+134h], xmm5
00007FF667E414D7  mulss        xmm5, xmm0
00007FF667E414DB  movss        dword ptr [rsp+58h], xmm5
00007FF667E414E1  mulss        xmm0, xmm6
00007FF667E414E5  movss        dword ptr [rsp+5Ch], xmm0
00007FF667E414EB  movss        dword ptr [rsp+138h], xmm6
00007FF667E414F4  movss        dword ptr [rsp+13Ch], xmm3
00007FF667E414FD  movss        dword ptr [rsp+140h], xmm7
00007FF667E41506  mov          dword ptr [rsp+14Ch], eax
00007FF667E4150D  mov          qword ptr [rsp+144h], rcx
00007FF667E41515  movaps       xmmword ptr [rsp+150h], xmm1
00007FF667E4151D  movaps       xmmword ptr [rsp+160h], xmm2
00007FF667E41525  mov          rcx, qword ptr [00007FF667E631C8h]
00007FF667E4152C  test         rcx, rcx
00007FF667E4152F  jne          static void Fabled_Engine::main+339h (00007FF667E41549h)
00007FF667E41531  call         GetProcessHeap (00007FF667E58EBCh)
00007FF667E41536  test         rax, rax
00007FF667E41539  je           static void Fabled_Engine::main+687h (00007FF667E41897h)
00007FF667E4153F  mov          rcx, rax
00007FF667E41542  mov          qword ptr [00007FF667E631C8h], rax
00007FF667E41549  mov          r8d, 100h
00007FF667E4154F  xor          edx, edx
00007FF667E41551  call         HeapAlloc (00007FF667E58EC2h)
00007FF667E41556  test         rax, rax
00007FF667E41559  je           static void Fabled_Engine::main+687h (00007FF667E41897h)
00007FF667E4155F  mov          rdi, rax
00007FF667E41562  lea          rdx, [rsp+70h]
00007FF667E41567  mov          r8d, 100h
00007FF667E4156D  mov          rcx, rax
00007FF667E41570  call         memcpy (00007FF667E59FA7h)
00007FF667E41575  mov          rcx, qword ptr [00007FF667E631C8h]
00007FF667E4157C  test         rcx, rcx
00007FF667E4157F  jne          static void Fabled_Engine::main+389h (00007FF667E41599h)
00007FF667E41581  call         GetProcessHeap (00007FF667E58EBCh)
00007FF667E41586  test         rax, rax
00007FF667E41589  je           static void Fabled_Engine::main+68Eh (00007FF667E4189Eh)
00007FF667E4158F  mov          rcx, rax
00007FF667E41592  mov          qword ptr [00007FF667E631C8h], rax
00007FF667E41599  mov          r8d, 30h
00007FF667E4159F  xor          edx, edx
00007FF667E415A1  call         HeapAlloc (00007FF667E58EC2h)
00007FF667E415A6  test         rax, rax
00007FF667E415A9  je           static void Fabled_Engine::main+68Eh (00007FF667E4189Eh)
00007FF667E415AF  mov          rbx, rax
00007FF667E415B2  movups       xmm0, xmmword ptr [00007FF667E5DAB0h]
00007FF667E415B9  movups       xmmword ptr [rax+20h], xmm0
00007FF667E415BD  movups       xmm0, xmmword ptr [00007FF667E5DAA0h]
00007FF667E415C4  movups       xmmword ptr [rax+10h], xmm0
00007FF667E415C8  movups       xmm0, xmmword ptr [00007FF667E5DA90h]
00007FF667E415CF  movups       xmmword ptr [rax], xmm0
00007FF667E415D2  mov          rcx, qword ptr [00007FF667E631C8h]
00007FF667E415D9  test         rcx, rcx
00007FF667E415DC  jne          static void Fabled_Engine::main+3E6h (00007FF667E415F6h)
00007FF667E415DE  call         GetProcessHeap (00007FF667E58EBCh)
00007FF667E415E3  test         rax, rax
00007FF667E415E6  je           static void Fabled_Engine::main+69Fh (00007FF667E418AFh)
00007FF667E415EC  mov          rcx, rax
00007FF667E415EF  mov          qword ptr [00007FF667E631C8h], rax
00007FF667E415F6  mov          r8d, 40h
00007FF667E415FC  xor          edx, edx
00007FF667E415FE  call         HeapAlloc (00007FF667E58EC2h)
00007FF667E41603  test         rax, rax
00007FF667E41606  je           static void Fabled_Engine::main+69Fh (00007FF667E418AFh)
00007FF667E4160C  mov          rsi, rax
00007FF667E4160F  mov          qword ptr [rax], rdi
00007FF667E41612  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000004 (00007FF667E5C550h)]
00007FF667E41619  movups       xmmword ptr [rax+8h], xmm0
00007FF667E4161D  add          rax, 18h
00007FF667E41621  mov          dword ptr [rsi+18h], 0h
00007FF667E41628  mov          qword ptr [rsi+20h], rbx
00007FF667E4162C  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000006 (00007FF667E5C560h)]
00007FF667E41633  movups       xmmword ptr [rsi+28h], xmm0