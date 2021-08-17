# --------------- IcoSphere Dissassembly -------------------
00007FF7CA621335  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA62133C  test         rcx, rcx
00007FF7CA62133F  jne          static void Fabled_Engine::main+99h (00007FF7CA621359h)
00007FF7CA621341  call         GetProcessHeap (00007FF7CA6398C2h)
00007FF7CA621346  test         rax, rax
00007FF7CA621349  je           static void Fabled_Engine::main+ACDh (00007FF7CA621D8Dh)
00007FF7CA62134F  mov          rcx, rax
00007FF7CA621352  mov          qword ptr [00007FF7CA6441C8h], rax
00007FF7CA621359  mov          r8d, 90h
00007FF7CA62135F  xor          edx, edx
00007FF7CA621361  call         HeapAlloc (00007FF7CA6398C8h)
00007FF7CA621366  test         rax, rax
00007FF7CA621369  je           static void Fabled_Engine::main+ACDh (00007FF7CA621D8Dh)
00007FF7CA62136F  movaps       xmm0, xmmword ptr [__xmm@3f06963e000000003f59c433bf06963e (00007FF7CA63D530h)]
00007FF7CA621376  movups       xmmword ptr [rax], xmm0
00007FF7CA621379  movaps       xmm0, xmmword ptr [__xmm@bf59c433bf06963e000000003f59c433 (00007FF7CA63D540h)]
00007FF7CA621380  movups       xmmword ptr [rax+10h], xmm0
00007FF7CA621384  movaps       xmm0, xmmword ptr [__xmm@00000000bf59c4333f06963e00000000 (00007FF7CA63D550h)]
00007FF7CA62138B  movups       xmmword ptr [rax+20h], xmm0
00007FF7CA62138F  movaps       xmm1, xmmword ptr [__xmm@000000003f59c433bf06963e00000000 (00007FF7CA63D560h)]
00007FF7CA621396  movups       xmmword ptr [rax+30h], xmm1
00007FF7CA62139A  movaps       xmm1, xmmword ptr [__xmm@bf06963e000000003f59c4333f06963e (00007FF7CA63D570h)]
00007FF7CA6213A1  movups       xmmword ptr [rax+40h], xmm1
00007FF7CA6213A5  movaps       xmm1, xmmword ptr [__xmm@bf59c4333f06963e00000000bf59c433 (00007FF7CA63D580h)]
00007FF7CA6213AC  movups       xmmword ptr [rax+50h], xmm1
00007FF7CA6213B0  movaps       xmm1, xmmword ptr [__xmm@3f59c433bf06963e000000003f59c433 (00007FF7CA63D590h)]
00007FF7CA6213B7  movups       xmmword ptr [rax+60h], xmm1
00007FF7CA6213BB  movups       xmmword ptr [rax+70h], xmm0
00007FF7CA6213BF  movaps       xmm0, xmmword ptr [__xmm@3f06963e00000000bf59c433bf06963e (00007FF7CA63D5A0h)]
00007FF7CA6213C6  movups       xmmword ptr [rax+80h], xmm0
00007FF7CA6213CD  mov          qword ptr [rsp+30h], rax
00007FF7CA6213D2  movaps       xmm0, xmmword ptr [__xmm@00000000000000240000000000000024 (00007FF7CA63D5B0h)]
00007FF7CA6213D9  movups       xmmword ptr [rsp+38h], xmm0
00007FF7CA6213DE  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA6213E5  test         rcx, rcx
00007FF7CA6213E8  jne          static void Fabled_Engine::main+142h (00007FF7CA621402h)
00007FF7CA6213EA  call         GetProcessHeap (00007FF7CA6398C2h)
00007FF7CA6213EF  test         rax, rax
00007FF7CA6213F2  je           static void Fabled_Engine::main+AD4h (00007FF7CA621D94h)
00007FF7CA6213F8  mov          rcx, rax
00007FF7CA6213FB  mov          qword ptr [00007FF7CA6441C8h], rax
00007FF7CA621402  mov          r8d, 1E0h
00007FF7CA621408  xor          edx, edx
00007FF7CA62140A  call         HeapAlloc (00007FF7CA6398C8h)
00007FF7CA62140F  test         rax, rax
00007FF7CA621412  je           static void Fabled_Engine::main+AD4h (00007FF7CA621D94h)
00007FF7CA621418  mov          rbx, rax
00007FF7CA62141B  movaps       xmm0, xmmword ptr [__xmm@000000000000000b0000000000000000 (00007FF7CA63D5C0h)]
00007FF7CA621422  movups       xmmword ptr [rax], xmm0
00007FF7CA621425  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000005 (00007FF7CA63D5D0h)]
00007FF7CA62142C  movups       xmmword ptr [rax+10h], xmm0
00007FF7CA621430  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000005 (00007FF7CA63D5E0h)]
00007FF7CA621437  movups       xmmword ptr [rax+20h], xmm0
00007FF7CA62143B  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000000 (00007FF7CA63D5F0h)]
00007FF7CA621442  movups       xmmword ptr [rax+30h], xmm0
00007FF7CA621446  movaps       xmm0, xmmword ptr [__xmm@00000000000000000000000000000007 (00007FF7CA63D600h)]
00007FF7CA62144D  movups       xmmword ptr [rax+40h], xmm0
00007FF7CA621451  movaps       xmm0, xmmword ptr [__xmm@000000000000000a0000000000000007 (00007FF7CA63D610h)]
00007FF7CA621458  movups       xmmword ptr [rax+50h], xmm0
00007FF7CA62145C  movaps       xmm0, xmmword ptr [__xmm@000000000000000a0000000000000000 (00007FF7CA63D620h)]
00007FF7CA621463  movups       xmmword ptr [rax+60h], xmm0
00007FF7CA621467  movaps       xmm0, xmmword ptr [__xmm@0000000000000001000000000000000b (00007FF7CA63D630h)]
00007FF7CA62146E  movups       xmmword ptr [rax+70h], xmm0
00007FF7CA621472  movaps       xmm0, xmmword ptr [__xmm@00000000000000090000000000000005 (00007FF7CA63D640h)]
00007FF7CA621479  movups       xmmword ptr [rax+80h], xmm0
00007FF7CA621480  movaps       xmm0, xmmword ptr [__xmm@000000000000000b0000000000000005 (00007FF7CA63D650h)]
00007FF7CA621487  movups       xmmword ptr [rax+90h], xmm0
00007FF7CA62148E  movaps       xmm0, xmmword ptr [__xmm@000000000000000b0000000000000004 (00007FF7CA63D660h)]
00007FF7CA621495  movups       xmmword ptr [rax+A0h], xmm0
00007FF7CA62149C  movaps       xmm0, xmmword ptr [__xmm@0000000000000002000000000000000a (00007FF7CA63D670h)]
00007FF7CA6214A3  movups       xmmword ptr [rax+B0h], xmm0
00007FF7CA6214AA  movaps       xmm0, xmmword ptr [__xmm@0000000000000007000000000000000a (00007FF7CA63D680h)]
00007FF7CA6214B1  movups       xmmword ptr [rax+C0h], xmm0
00007FF7CA6214B8  movaps       xmm0, xmmword ptr [__xmm@00000000000000070000000000000006 (00007FF7CA63D690h)]
00007FF7CA6214BF  movups       xmmword ptr [rax+D0h], xmm0
00007FF7CA6214C6  movaps       xmm0, xmmword ptr [__xmm@00000000000000080000000000000001 (00007FF7CA63D6A0h)]
00007FF7CA6214CD  movups       xmmword ptr [rax+E0h], xmm0
00007FF7CA6214D4  movaps       xmm0, xmmword ptr [__xmm@00000000000000090000000000000003 (00007FF7CA63D6B0h)]
00007FF7CA6214DB  movups       xmmword ptr [rax+F0h], xmm0
00007FF7CA6214E2  movaps       xmm0, xmmword ptr [__xmm@00000000000000030000000000000004 (00007FF7CA63D6C0h)]
00007FF7CA6214E9  movups       xmmword ptr [rax+100h], xmm0
00007FF7CA6214F0  movaps       xmm0, xmmword ptr [__xmm@00000000000000020000000000000004 (00007FF7CA63D6D0h)]
00007FF7CA6214F7  movups       xmmword ptr [rax+110h], xmm0
00007FF7CA6214FE  movaps       xmm0, xmmword ptr [__xmm@00000000000000020000000000000003 (00007FF7CA63D6E0h)]
00007FF7CA621505  movups       xmmword ptr [rax+120h], xmm0
00007FF7CA62150C  movaps       xmm0, xmmword ptr [__xmm@00000000000000030000000000000006 (00007FF7CA63D6F0h)]
00007FF7CA621513  movups       xmmword ptr [rax+130h], xmm0
00007FF7CA62151A  movaps       xmm0, xmmword ptr [__xmm@00000000000000080000000000000006 (00007FF7CA63D700h)]
00007FF7CA621521  movups       xmmword ptr [rax+140h], xmm0
00007FF7CA621528  movaps       xmm0, xmmword ptr [__xmm@00000000000000080000000000000003 (00007FF7CA63D710h)]
00007FF7CA62152F  movups       xmmword ptr [rax+150h], xmm0
00007FF7CA621536  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000009 (00007FF7CA63D720h)]
00007FF7CA62153D  movups       xmmword ptr [rax+160h], xmm0
00007FF7CA621544  movaps       xmm0, xmmword ptr [__xmm@00000000000000050000000000000009 (00007FF7CA63D730h)]
00007FF7CA62154B  movups       xmmword ptr [rax+170h], xmm0
00007FF7CA621552  movaps       xmm0, xmmword ptr [__xmm@00000000000000040000000000000002 (00007FF7CA63D740h)]
00007FF7CA621559  movups       xmmword ptr [rax+180h], xmm0
00007FF7CA621560  movaps       xmm0, xmmword ptr [__xmm@0000000000000006000000000000000b (00007FF7CA63D750h)]
00007FF7CA621567  movups       xmmword ptr [rax+190h], xmm0
00007FF7CA62156E  movaps       xmm0, xmmword ptr [__xmm@000000000000000a0000000000000002 (00007FF7CA63D760h)]
00007FF7CA621575  movups       xmmword ptr [rax+1A0h], xmm0
00007FF7CA62157C  movaps       xmm0, xmmword ptr [__xmm@00000000000000060000000000000008 (00007FF7CA63D770h)]
00007FF7CA621583  movups       xmmword ptr [rax+1B0h], xmm0
00007FF7CA62158A  movaps       xmm0, xmmword ptr [__xmm@00000000000000090000000000000007 (00007FF7CA63D780h)]
00007FF7CA621591  movups       xmmword ptr [rax+1C0h], xmm0
00007FF7CA621598  movaps       xmm0, xmmword ptr [__xmm@00000000000000010000000000000008 (00007FF7CA63D790h)]
00007FF7CA62159F  movups       xmmword ptr [rax+1D0h], xmm0
00007FF7CA6215A6  xorps        xmm0, xmm0
00007FF7CA6215A9  movups       xmmword ptr [rsp+80h], xmm0
00007FF7CA6215B1  mov          qword ptr [rsp+70h], 0h
00007FF7CA6215BA  lea          rax, [00007FF7CA63EE60h]
00007FF7CA6215C1  mov          qword ptr [rsp+78h], rax
00007FF7CA6215C6  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA6215CD  test         rcx, rcx
00007FF7CA6215D0  jne          static void Fabled_Engine::main+32Ah (00007FF7CA6215EAh)
00007FF7CA6215D2  call         GetProcessHeap (00007FF7CA6398C2h)
00007FF7CA6215D7  test         rax, rax
00007FF7CA6215DA  je           static void Fabled_Engine::main+B9Fh (00007FF7CA621E5Fh)
00007FF7CA6215E0  mov          rcx, rax
00007FF7CA6215E3  mov          qword ptr [00007FF7CA6441C8h], rax
00007FF7CA6215EA  mov          esi, 780h
00007FF7CA6215EF  mov          r8d, 780h
00007FF7CA6215F5  mov          edx, 8h
00007FF7CA6215FA  call         HeapAlloc (00007FF7CA6398C8h)
00007FF7CA6215FF  test         rax, rax
00007FF7CA621602  je           static void Fabled_Engine::main+BABh (00007FF7CA621E6Bh)
00007FF7CA621608  mov          r13, rax
00007FF7CA62160B  xor          esi, esi
00007FF7CA62160D  mov          qword ptr [rsp+68h], rbx
00007FF7CA621612  nop          word ptr cs:[rax+rax*1], ax
00007FF7CA62161C  nop          dword ptr [rax], eax
00007FF7CA621620  mov          rdi, qword ptr [rbx+rsi*2]
00007FF7CA621624  mov          rbp, qword ptr [rbx+rsi*2+8h]
00007FF7CA621629  mov          rbx, qword ptr [rbx+rsi*2+10h]
00007FF7CA62162E  mov          rcx, rdi
00007FF7CA621631  mov          rdx, rbp
00007FF7CA621634  lea          r12, [rsp+30h]
00007FF7CA621639  mov          r8, r12
00007FF7CA62163C  lea          r15, [rsp+70h]
00007FF7CA621641  mov          r9, r15
00007FF7CA621644  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF7CA625C50h)
00007FF7CA621649  mov          r14, rax
00007FF7CA62164C  mov          rcx, rbp
00007FF7CA62164F  mov          rdx, rbx
00007FF7CA621652  mov          r8, r12
00007FF7CA621655  mov          r9, r15
00007FF7CA621658  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF7CA625C50h)
00007FF7CA62165D  mov          r15, rax
00007FF7CA621660  mov          rcx, rbx
00007FF7CA621663  mov          rdx, rdi
00007FF7CA621666  mov          r8, r12
00007FF7CA621669  lea          r9, [rsp+70h]
00007FF7CA62166E  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF7CA625C50h)
00007FF7CA621673  cmp          rsi, FCh
00007FF7CA62167A  je           static void Fabled_Engine::main+AB0h (00007FF7CA621D70h)
00007FF7CA621680  mov          qword ptr [r13+rsi*8], rdi
00007FF7CA621685  mov          qword ptr [r13+rsi*8+8h], r14
00007FF7CA62168A  mov          qword ptr [r13+rsi*8+10h], rax
00007FF7CA62168F  mov          qword ptr [r13+rsi*8+18h], rbp
00007FF7CA621694  mov          qword ptr [r13+rsi*8+20h], r15
00007FF7CA621699  mov          qword ptr [r13+rsi*8+28h], r14
00007FF7CA62169E  mov          qword ptr [r13+rsi*8+30h], rbx
00007FF7CA6216A3  mov          qword ptr [r13+rsi*8+38h], rax
00007FF7CA6216A8  mov          qword ptr [r13+rsi*8+40h], r15
00007FF7CA6216AD  mov          qword ptr [r13+rsi*8+48h], r14
00007FF7CA6216B2  mov          qword ptr [r13+rsi*8+50h], r15
00007FF7CA6216B7  mov          qword ptr [r13+rsi*8+58h], rax
00007FF7CA6216BC  add          rsi, Ch
00007FF7CA6216C0  cmp          rsi, F0h
00007FF7CA6216C7  mov          rbx, qword ptr [rsp+68h]
00007FF7CA6216CC  jne          static void Fabled_Engine::main+360h (00007FF7CA621620h)
00007FF7CA6216D2  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA6216D9  xor          edx, edx
00007FF7CA6216DB  mov          r8, rbx
00007FF7CA6216DE  call         HeapFree (00007FF7CA6398CEh)
00007FF7CA6216E3  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA6216EA  test         rcx, rcx
00007FF7CA6216ED  jne          static void Fabled_Engine::main+447h (00007FF7CA621707h)
00007FF7CA6216EF  call         GetProcessHeap (00007FF7CA6398C2h)
00007FF7CA6216F4  test         rax, rax
00007FF7CA6216F7  je           static void Fabled_Engine::main+BA6h (00007FF7CA621E66h)
00007FF7CA6216FD  mov          rcx, rax
00007FF7CA621700  mov          qword ptr [00007FF7CA6441C8h], rax
00007FF7CA621707  mov          esi, 1E00h
00007FF7CA62170C  mov          r8d, 1E00h
00007FF7CA621712  mov          edx, 8h
00007FF7CA621717  call         HeapAlloc (00007FF7CA6398C8h)
00007FF7CA62171C  mov          qword ptr [rsp+68h], rax
00007FF7CA621721  test         rax, rax
00007FF7CA621724  je           static void Fabled_Engine::main+BABh (00007FF7CA621E6Bh)
00007FF7CA62172A  xor          esi, esi
00007FF7CA62172C  xor          r12d, r12d
00007FF7CA62172F  nop
00007FF7CA621730  mov          rbp, qword ptr [r13+rsi*2]
00007FF7CA621735  mov          rbx, qword ptr [r13+rsi*2+8h]
00007FF7CA62173A  mov          rdi, qword ptr [r13+rsi*2+10h]
00007FF7CA62173F  mov          rcx, rbp
00007FF7CA621742  mov          rdx, rbx
00007FF7CA621745  lea          r14, [rsp+30h]
00007FF7CA62174A  mov          r8, r14
00007FF7CA62174D  lea          r9, [rsp+70h]
00007FF7CA621752  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF7CA625C50h)
00007FF7CA621757  mov          r15, rax
00007FF7CA62175A  mov          rcx, rbx
00007FF7CA62175D  mov          rdx, rdi
00007FF7CA621760  mov          r8, r14
00007FF7CA621763  lea          r9, [rsp+70h]
00007FF7CA621768  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF7CA625C50h)
00007FF7CA62176D  mov          r14, rax
00007FF7CA621770  mov          rcx, rdi
00007FF7CA621773  mov          rdx, rbp
00007FF7CA621776  lea          r8, [rsp+30h]
00007FF7CA62177B  lea          r9, [rsp+70h]
00007FF7CA621780  call         static unsigned __int64 fabled_render::mesh::primitive::icosphere::IcoSphere::middle_point (00007FF7CA625C50h)
00007FF7CA621785  cmp          r12, 50h
00007FF7CA621789  je           static void Fabled_Engine::main+A96h (00007FF7CA621D56h)
00007FF7CA62178F  cmp          r12, 51h
00007FF7CA621793  je           static void Fabled_Engine::main+AB7h (00007FF7CA621D77h)
00007FF7CA621799  add          r12, 1h
00007FF7CA62179D  mov          rcx, qword ptr [rsp+68h]
00007FF7CA6217A2  mov          qword ptr [rcx+rsi*8], rbp
00007FF7CA6217A6  mov          qword ptr [rcx+rsi*8+8h], r15
00007FF7CA6217AB  mov          qword ptr [rcx+rsi*8+10h], rax
00007FF7CA6217B0  mov          qword ptr [rcx+rsi*8+18h], rbx
00007FF7CA6217B5  mov          qword ptr [rcx+rsi*8+20h], r14
00007FF7CA6217BA  mov          qword ptr [rcx+rsi*8+28h], r15
00007FF7CA6217BF  mov          qword ptr [rcx+rsi*8+30h], rdi
00007FF7CA6217C4  mov          qword ptr [rcx+rsi*8+38h], rax
00007FF7CA6217C9  mov          qword ptr [rcx+rsi*8+40h], r14
00007FF7CA6217CE  mov          qword ptr [rcx+rsi*8+48h], r15
00007FF7CA6217D3  mov          qword ptr [rcx+rsi*8+50h], r14
00007FF7CA6217D8  mov          qword ptr [rcx+rsi*8+58h], rax
00007FF7CA6217DD  add          rsi, Ch
00007FF7CA6217E1  cmp          r12, 50h
00007FF7CA6217E5  jne          static void Fabled_Engine::main+470h (00007FF7CA621730h)
00007FF7CA6217EB  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA6217F2  xor          edx, edx
00007FF7CA6217F4  mov          r8, r13
00007FF7CA6217F7  call         HeapFree (00007FF7CA6398CEh)
00007FF7CA6217FC  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA621803  test         rcx, rcx
00007FF7CA621806  jne          static void Fabled_Engine::main+560h (00007FF7CA621820h)
00007FF7CA621808  call         GetProcessHeap (00007FF7CA6398C2h)
00007FF7CA62180D  test         rax, rax
00007FF7CA621810  je           static void Fabled_Engine::main+AE5h (00007FF7CA621DA5h)
00007FF7CA621816  mov          rcx, rax
00007FF7CA621819  mov          qword ptr [00007FF7CA6441C8h], rax
00007FF7CA621820  mov          r8d, 788h
00007FF7CA621826  mov          edx, 8h
00007FF7CA62182B  call         HeapAlloc (00007FF7CA6398C8h)
00007FF7CA621830  test         rax, rax
00007FF7CA621833  je           static void Fabled_Engine::main+AE5h (00007FF7CA621DA5h)
00007FF7CA621839  mov          rbx, rax
00007FF7CA62183C  mov          r14, qword ptr [rsp+40h]
00007FF7CA621841  mov          r15, AAAAAAAAAAAAAAABh
00007FF7CA62184B  mov          rax, r14
00007FF7CA62184E  mul          r15
00007FF7CA621851  mov          rdi, qword ptr [rsp+30h]
00007FF7CA621856  shr          rdx, 1h
00007FF7CA621859  lea          rax, [rdx+rdx*2]
00007FF7CA62185D  cmp          rax, 3h
00007FF7CA621861  jb           static void Fabled_Engine::main+636h (00007FF7CA6218F6h)
00007FF7CA621867  add          rax, FFFFFFFFFFFFFFFDh
00007FF7CA62186B  mul          r15
00007FF7CA62186E  mov          r12, rdx
00007FF7CA621871  shr          r12, 1h
00007FF7CA621874  add          r12, 1h
00007FF7CA621878  lea          rbp, [rdi+8h]
00007FF7CA62187C  xor          esi, esi
00007FF7CA62187E  movss        xmm8, dword ptr [__real@3ea2f983 (00007FF7CA63D7A0h)]
00007FF7CA621887  movss        xmm7, dword ptr [__real@3f000000 (00007FF7CA63D7A4h)]
00007FF7CA62188F  movss        xmm9, dword ptr [__real@3e22f983 (00007FF7CA63D7A8h)]
00007FF7CA621898  nop          dword ptr [rax+rax*1], eax
00007FF7CA6218A0  movss        xmm1, dword ptr [rbp-8h]
00007FF7CA6218A5  movss        xmm0, dword ptr [rbp]
00007FF7CA6218AA  call         atan2f (00007FF7CA63A9E9h)
00007FF7CA6218AF  movaps       xmm6, xmm0
00007FF7CA6218B2  movss        xmm0, dword ptr [rbp-4h]
00007FF7CA6218B7  call         asinf (00007FF7CA63A9EFh)
00007FF7CA6218BC  cmp          rsi, F1h
00007FF7CA6218C3  je           static void Fabled_Engine::main+A96h (00007FF7CA621D56h)
00007FF7CA6218C9  mulss        xmm0, xmm8
00007FF7CA6218CE  addss        xmm0, xmm7
00007FF7CA6218D2  mulss        xmm6, xmm9
00007FF7CA6218D7  addss        xmm6, xmm7
00007FF7CA6218DB  movss        dword ptr [rbx+rsi*8], xmm6
00007FF7CA6218E0  movss        dword ptr [rbx+rsi*8+4h], xmm0
00007FF7CA6218E6  lea          rax, [rsi+1h]
00007FF7CA6218EA  add          rbp, Ch
00007FF7CA6218EE  mov          rsi, rax
00007FF7CA6218F1  cmp          r12, rax
00007FF7CA6218F4  jne          static void Fabled_Engine::main+5E0h (00007FF7CA6218A0h)
00007FF7CA6218F6  mov          rcx, qword ptr [00007FF7CA63ECE0h]
00007FF7CA6218FD  mov          qword ptr [rsp+48h], rcx
00007FF7CA621902  xorps        xmm8, xmm8
00007FF7CA621906  movups       xmmword ptr [rsp+50h], xmm8
00007FF7CA62190C  mov          rax, r14
00007FF7CA62190F  mul          r15
00007FF7CA621912  shr          rdx, 1h
00007FF7CA621915  cmp          rdx, F1h
00007FF7CA62191C  mov          ebp, F1h
00007FF7CA621921  cmovb        rbp, rdx
00007FF7CA621925  test         rbp, rbp
00007FF7CA621928  je           static void Fabled_Engine::main+750h (00007FF7CA621A10h)
00007FF7CA62192E  add          rbp, FFFFFFFFFFFFFFFFh
00007FF7CA621932  add          rdi, 8h
00007FF7CA621936  xor          edx, edx
00007FF7CA621938  lea          r14, [rsp+48h]
00007FF7CA62193D  xor          eax, eax
00007FF7CA62193F  xor          esi, esi
00007FF7CA621941  nop          word ptr cs:[rax+rax*1], ax
00007FF7CA62194B  nop          dword ptr [rax+rax*1], eax
00007FF7CA621950  movss        xmm7, dword ptr [rdi-8h]
00007FF7CA621955  movss        xmm6, dword ptr [rdi-4h]
00007FF7CA62195A  movss        xmm9, dword ptr [rdi]
00007FF7CA62195F  movss        xmm10, dword ptr [rbx+rsi*8]
00007FF7CA621965  movss        xmm11, dword ptr [rbx+rsi*8+4h]
00007FF7CA62196C  movaps       xmmword ptr [rsp+A0h], xmm8
00007FF7CA621975  movaps       xmmword ptr [rsp+B0h], xmm8
00007FF7CA62197E  cmp          rax, rdx
00007FF7CA621981  je           static void Fabled_Engine::main+739h (00007FF7CA6219F9h)
00007FF7CA621983  shl          rax, 6h
00007FF7CA621987  movss        dword ptr [rcx+rax*1], xmm7
00007FF7CA62198C  movss        dword ptr [rcx+rax*1+4h], xmm6
00007FF7CA621992  movss        dword ptr [rcx+rax*1+8h], xmm9
00007FF7CA621999  movss        dword ptr [rcx+rax*1+Ch], xmm10
00007FF7CA6219A0  movss        dword ptr [rcx+rax*1+10h], xmm11
00007FF7CA6219A7  movss        dword ptr [rcx+rax*1+14h], xmm7
00007FF7CA6219AD  movss        dword ptr [rcx+rax*1+18h], xmm6
00007FF7CA6219B3  movss        dword ptr [rcx+rax*1+1Ch], xmm9
00007FF7CA6219BA  movaps       xmm0, xmmword ptr [rsp+A0h]
00007FF7CA6219C2  movaps       xmmword ptr [rcx+rax*1+20h], xmm0
00007FF7CA6219C7  movaps       xmm0, xmmword ptr [rsp+B0h]
00007FF7CA6219CF  movaps       xmmword ptr [rcx+rax*1+30h], xmm0
00007FF7CA6219D4  mov          rax, qword ptr [rsp+58h]
00007FF7CA6219D9  add          rax, 1h
00007FF7CA6219DD  mov          qword ptr [rsp+58h], rax
00007FF7CA6219E2  cmp          rbp, rsi
00007FF7CA6219E5  je           static void Fabled_Engine::main+750h (00007FF7CA621A10h)
00007FF7CA6219E7  add          rsi, 1h
00007FF7CA6219EB  mov          rdx, qword ptr [rsp+50h]
00007FF7CA6219F0  add          rdi, Ch
00007FF7CA6219F4  jmp          static void Fabled_Engine::main+690h (00007FF7CA621950h)
00007FF7CA6219F9  mov          rcx, r14
00007FF7CA6219FC  call         static void alloc::raw_vec::{{impl}}::reserve::do_reserve_and_handle<fabled_render::mesh::container::vertex::Vertex,alloc::alloc::Global> (00007FF7CA63B930h)
00007FF7CA621A01  mov          rcx, qword ptr [rsp+48h]
00007FF7CA621A06  mov          rax, qword ptr [rsp+58h]
00007FF7CA621A0B  jmp          static void Fabled_Engine::main+6C3h (00007FF7CA621983h)
00007FF7CA621A10  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA621A17  test         rcx, rcx
00007FF7CA621A1A  jne          static void Fabled_Engine::main+774h (00007FF7CA621A34h)
00007FF7CA621A1C  call         GetProcessHeap (00007FF7CA6398C2h)
00007FF7CA621A21  test         rax, rax
00007FF7CA621A24  je           static void Fabled_Engine::main+AF6h (00007FF7CA621DB6h)
00007FF7CA621A2A  mov          rcx, rax
00007FF7CA621A2D  mov          qword ptr [00007FF7CA6441C8h], rax
00007FF7CA621A34  mov          r8d, 40h
00007FF7CA621A3A  xor          edx, edx
00007FF7CA621A3C  call         HeapAlloc (00007FF7CA6398C8h)
00007FF7CA621A41  test         rax, rax
00007FF7CA621A44  je           static void Fabled_Engine::main+AF6h (00007FF7CA621DB6h)
00007FF7CA621A4A  mov          r14, rax
00007FF7CA621A4D  mov          rax, qword ptr [rsp+58h]
00007FF7CA621A52  mov          qword ptr [r14+10h], rax
00007FF7CA621A56  movups       xmm0, xmmword ptr [rsp+48h]
00007FF7CA621A5B  movups       xmmword ptr [r14], xmm0
00007FF7CA621A5F  mov          rax, qword ptr [rsp+68h]
00007FF7CA621A64  mov          qword ptr [r14+20h], rax
00007FF7CA621A68  movaps       xmm0, xmmword ptr [__xmm@00000000000003c000000000000003c0 (00007FF7CA63D7B0h)]
00007FF7CA621A6F  movups       xmmword ptr [r14+28h], xmm0
00007FF7CA621A74  mov          dword ptr [r14+18h], 0h
00007FF7CA621A7C  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA621A83  xor          edx, edx
00007FF7CA621A85  mov          r8, rbx
00007FF7CA621A88  call         HeapFree (00007FF7CA6398CEh)
00007FF7CA621A8D  mov          rcx, qword ptr [rsp+70h]
00007FF7CA621A92  test         rcx, rcx
00007FF7CA621A95  je           static void Fabled_Engine::main+802h (00007FF7CA621AC2h)
00007FF7CA621A97  lea          rax, [rcx+1h]
00007FF7CA621A9B  mov          edx, 10h
00007FF7CA621AA0  mul          rdx
00007FF7CA621AA3  add          rcx, rax
00007FF7CA621AA6  cmp          rcx, FFFFFFFFFFFFFFEFh
00007FF7CA621AAA  je           static void Fabled_Engine::main+802h (00007FF7CA621AC2h)
00007FF7CA621AAC  mov          r8, qword ptr [rsp+78h]
00007FF7CA621AB1  sub          r8, rax
00007FF7CA621AB4  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA621ABB  xor          edx, edx
00007FF7CA621ABD  call         HeapFree (00007FF7CA6398CEh)
00007FF7CA621AC2  mov          rax, qword ptr [rsp+38h]
00007FF7CA621AC7  test         rax, rax
00007FF7CA621ACA  je           static void Fabled_Engine::main+82Dh (00007FF7CA621AEDh)
00007FF7CA621ACC  shl          rax, 2h
00007FF7CA621AD0  test         rax, rax
00007FF7CA621AD3  je           static void Fabled_Engine::main+82Dh (00007FF7CA621AEDh)
00007FF7CA621AD5  mov          r8, qword ptr [rsp+30h]
00007FF7CA621ADA  test         r8, r8
00007FF7CA621ADD  je           static void Fabled_Engine::main+82Dh (00007FF7CA621AEDh)
00007FF7CA621ADF  mov          rcx, qword ptr [00007FF7CA6441C8h]
00007FF7CA621AE6  xor          edx, edx
00007FF7CA621AE8  call         HeapFree (00007FF7CA6398CEh)