# Writing an OS in Rust

- https://os.phil-opp.com/

- https://os.phil-opp.com/freestanding-rust-binary/

- code
  - 러스트로 작은 OS만들기
  - https://github.com/phil-opp/blog_os/



# Hello World with Bare Metal and QEMU

- https://developer.arm.com/community/arm-community-blogs/b/architectures-and-processors-blog/posts/hello-world-with-bare-metal-and-qemu

# Official QEMU mirror.
- https://github.com/qemu/qemu
  - https://www.qemu.org/contribute/

- Tutorial
  - https://github.com/TunaCici/QEMU_Starter/blob/main/Documents/README_2_HelloWorld.md

# Creating an OS from Scratch(Rust)
- https://youtube.com/playlist?list=PLDeRIkiONWGuF8TU5RD3RIUd3334cwanB&si=xu2ObkhF3y6Dmlr4

- OSdev.
  - https://wiki.osdev.org/Expanded_Main_Page

- CPU 작동방법?
  - https://cpu.land/

<hr />

# pintoOS만드는거 pdf 로 잘 정리됨
- https://pintos-os.org/SIGCSE2009-Pintos.pdf

# pintos순서대로 공부하기 좋다.
- https://www.scs.stanford.edu/25sp-cs212/pintos/pintos.html#SEC_Top


<hr />

# Beginners in OSDev are strongly recommended not to skip this.

- Due to the complexity of the task, writing an OS requires a high level of familiarity and experience with programming and computer science concepts.

- Developing an OS requires knowledge on the following areas:

- 1. **Basic Computer Science**: Intimate familiarity with hexadecimal and binary notation as well as boolean logic and similar fundamental knowledge, like data structures, their construction and manipulation, searching and sorting algorithms, abstract programming concepts.

- 2. **Language and Vocabulary**: Ability to read and write (technical) English. Virtually all technical documentation is in English, including most of the resources on the web (like this Wiki and the forum are). Correct terminology is imperative for receiving proper assistance.

- 3. **Language and Vocabulary, pt. 2**: Most operating systems featured on this site, as well as most of the code snippets and examples, are written in C (or C++). Due to this, a strong understanding of C is required for OS development.

- 4. **Assembly**: Knowledge about the low-level language Assembly. Resources for learning include books and school/college courses. Writing user-space code serves as practice. A strong understanding of Assembly is required, even if one plans to write most of the operating system in a higher-level language.

- 5. **Programming experience**: Developing OS projects requires experience with developing user-space programs and a good understanding of the programming language that will be used before proceeding with a project. Knowledge of concepts such as version control and debugging is required.

- 6. **Programming Practices**: Ability to write both code and user documentation, and be able to document all aspects of code and design, even if the project is purely for personal use. Programmers must learn and use appropriate Code Management practices, including setting up and using an off-site repository (e.g. Savannah, GitHub, GitLab, Heroku) in order to prevent any issues during development.

- 7. **UNIX experience**: Most OS development tools are developed natively for Unix. The Linux kernel is a reference for how things can be done, and many of the hobby operating systems have some resemblance to Unix. Having experience with the Unix command line (preferably Bash or ksh) is a very important requirement. (Cygwin provides an easy-to-install Unix command line for Windows.) For Windows users, this can be accomplished by using a virtualizer (see below) to host a virtual system, without the need to re-partition the home system. Windows Subsystem for Linux (WSL) also allows access to the Unix command line. macOS users can use the Terminal--macOS is built on a Unix variant kernel (a combination of the Mach and BSD kernels), if Xcode and an emulator or virtualizer are installed, no further tools are necessary (a cross-compiler is still required); the default Terminal shell is the Z Shell, but C shell and KornShell are available as well.

- 8. **Toolchain**: Knowledge of the behavioral details of the compiler, assembler, linker, and make utility. Being able to understand emitted warnings and errors. Tool documentations are the first source that should be referenced. Answer to most beginner's questions about GCC, GNU as, NASM, GNU ld, Visual Studio, and GRUB can be found online.

- 9. **Emulators and Virtualizers**: Familiarity with tools such as Bochs, VirtualBox, QEMU, or Virtual PC. These tools provide a buffer between real hardware and the test system. Knowledge of these tools is required prior to starting OS development.

- 10. **Executable Formats**: Kernel space programming has many additional requirements unknown to application development. Being able to parse executable binaries is an example. One should familiarize themselves with Executable File Types, their internal structure, and how a linker generates them.

- 11. **The Platform**: Read the manuals for the processor that the code will be written for. They contain the information required to design the kernel.

- 12. **The Concept**: Technical knowledge of currently existing operating systems is beneficial as these are built on the same principles required for any successful OS project.

## OSDev 초보자라면 이 과정을 꼭 수강할 것을 권장합니다.

- 작업의 복잡성 때문에 운영체제를 작성하려면 프로그래밍과 컴퓨터 과학 개념에 대한 높은 수준의 이해와 경험이 필요합니다.

- 운영체제를 개발하려면 다음 분야에 대한 지식이 필요합니다:

- 1. **기초 컴퓨터 과학**: 16진수와 2진수 표기법, 불리언 논리 및 데이터 구조, 생성과 조작, 검색 및 정렬 알고리즘, 추상 프로그래밍 개념 등 기본 지식에 익숙해집니다.

- 2. **언어 및 어휘**: (전문) 영어를 읽고 쓸 수 있는 능력. 사실상 모든 기술 문서가 영어로 되어 있으며, 이 위키나 포럼 같은 웹상의 대부분 자료도 마찬가지입니다. 정확한 용어 사용은 적절한 도움을 받기 위해 반드시 필요합니다.

- 3. **언어 및 어휘, 2부**: 이 사이트에 소개된 대부분의 운영체제와 코드 예제 대부분은 C(또는 C++)로 작성되었습니다. 따라서 운영체제 개발을 위해서는 C에 대한 깊은 이해가 필요합니다.

- 4. **어셈블리**: 저수준 언어인 어셈블리에 대한 지식. 학습 자료로는 책과 학교/대학 강좌가 포함됩니다. 사용자 공간 코드를 작성하는 것은 연습이 됩니다. 운영체제의 대부분을 고급 언어로 작성할 계획이라 하더라도, 어셈블리에 대한 탄탄한 이해가 필요합니다.

- 5. **프로그래밍 경험**: 운영체제 프로젝트를 진행하려면 사용자 공간 프로그램 개발 경험이 필요하며, 프로젝트를 시작하기 전에 사용할 프로그래밍 언어에 대한 충분한 이해가 요구됩니다. 버전 관리와 디버깅 같은 개념에 대한 지식이 필요합니다.

- 6. **프로그래밍 실무**: 코드와 사용자 문서를 모두 작성할 수 있어야 하며, 프로젝트가 개인용이라 하더라도 코드와 설계의 모든 부분을 문서화할 수 있어야 합니다. 개발 중 발생할 수 있는 문제를 예방하기 위해, 프로그래머는 적절한 코드 관리 방법을 배우고 활용해야 하며, 여기에는 오프사이트 저장소(예: Savannah, GitHub, GitLab, Heroku)를 구축하고 사용하는 것이 포함됩니다.

- 7. **유닉스 경험**: 대부분의 운영체제 개발 도구는 유닉스용으로 네이티브 개발됩니다. 리눅스 커널은 작업을 수행하는 방식을 보여주는 기준이 되며, 많은 취미용 운영체제들이 유닉스와 어느 정도 닮아 있습니다. Unix 명령어 라인(가능하면 Bash 또는 ksh) 사용 경험이 매우 중요합니다. (Cygwin은 윈도우용으로 설치하기 쉬운 유닉스 명령줄을 제공합니다.) 윈도우 사용자라면, 홈 시스템을 다시 파티션하지 않고도 가상화 장치(아래 참조)를 이용해 가상 시스템을 호스팅함으로써 이를 해결할 수 있습니다. Windows Subsystem for Linux(WSL)는 유닉스 명령줄에 접근할 수 있게 해줍니다. macOS 사용자는 터미널을 사용할 수 있습니다—macOS는 Mach와 BSD 커널의 조합인 유닉스 계열 커널(변형 커널)을 기반으로 만들어졌으며, Xcode와 에뮬레이터 또는 가상화 도구가 설치되어 있다면, 추가 도구는 필요 없으며(여전히 크로스 컴파일러가 필요합니다), 기본 터미널 셸은 Z 셸이지만 C 셸과 KornShell도 사용할 수 있습니다.

- 8. **툴체인**: 컴파일러, 어셈블러, 링커, Make 유틸리티의 동작 방식에 대한 이해가 필요합니다. 발생한 경고와 오류를 이해할 수 있어야 합니다. 도구 문서는 참고해야 할 첫 번째 출처입니다. GCC, GNU as, NASM, GNU ld, Visual Studio, GRUB에 관한 대부분의 초보자 질문에 대한 답변은 온라인에서 찾을 수 있습니다.

- 9. **에뮬레이터 및 가상화 도구**: Bochs, VirtualBox, QEMU, Virtual PC 등의 도구에 익숙함. 이 도구들은 실제 하드웨어와 테스트 시스템 사이에 완충 역할을 합니다. 운영체제 개발을 시작하기 전에 이러한 도구들에 대한 지식이 필요합니다.

- 10. 실행 파일 형식: 커널 공간 프로그래밍은 애플리케이션 개발자들이 잘 모르는 추가 요구사항이 많습니다. 실행 파일을 파싱할 수 있다는 것이 그 예입니다. 실행 파일 유형과 그 내부 구조, 그리고 링커가 이를 생성하는 방식을 익혀야 합니다.

- 11. **플랫폼**: 코드가 작성될 프로세서의 매뉴얼을 읽으세요. 커널을 설계하는 데 필요한 정보를 포함하고 있습니다.

- 12. **개념**: 현재 운영 중인 운영체제에 대한 기술적 지식이 있으면 유리한데, 이는 모든 성공적인 OS 프로젝트에 필요한 동일한 원칙 위에 구축되기 때문입니다.
- https://wiki.osdev.org/Required_Knowledge
