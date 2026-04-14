#!/usr/bin/env python3
"""
从 virus.c 中提取 VIRUS_PAYLOAD 标记区间，按“普通字符累积 / 遇转义类字符拆分”规则
生成供 PAYLOAD[] 使用的分段文本，写入 payload 文件。
"""

from __future__ import annotations

from pathlib import Path

BEGIN_MARKER = "// VIRUS_PAYLOAD_BEGIN_MARKER"
END_MARKER = "// VIRUS_PAYLOAD_END_MARKER"

# 遇到这些字符时：先 flush 当前累积串，再把该字符的“可打印转义形式”作为单独一段入数组
_SPLIT_CHARS = frozenset("\n\t\r\"'\\")

# 单字符 -> 数组里存放的“两字符”表示（反斜杠 + 字母/符号），与 infect() 里 fputs 逻辑一致
_ESC_REP: dict[str, str] = {
    "\n": "\\n",
    "\t": "\\t",
    "\r": "\\r",
    '"': '\\"',
    "'": "\\'",
    "\\": "\\\\"
}

# 由分隔符拆出的「两字符转义写法」段：写入 payload 时按字面输出 "\n" 而非再套一层成 "\\n"
_ESC_ASCII_PARTS = frozenset(_ESC_REP.values())


def _extract_region(text: str) -> str:
    i = text.find(BEGIN_MARKER)
    j = text.find(END_MARKER)
    if i == -1:
        raise SystemExit(f"未找到开始标记: {BEGIN_MARKER!r}")
    if j == -1:
        raise SystemExit(f"未找到结束标记: {END_MARKER!r}")
    if j < i:
        raise SystemExit("结束标记出现在开始标记之前")
    return text[i : j + len(END_MARKER)]


def _split_payload(region: str) -> list[str]:
    parts: list[str] = []
    s: list[str] = []
    for ch in region:
        if ch in _SPLIT_CHARS:
            parts.append("".join(s))
            s.clear()
            parts.append(_ESC_REP[ch])
        else:
            s.append(ch)
    parts.append("".join(s))
    return parts

def _quote_payload_part(p: str) -> str:
    return '"' + p + '"'


def main() -> None:
    root = Path(__file__).resolve().parent
    virus_path = root / "virus.c"
    out_path = root / "payload"

    region = _extract_region(virus_path.read_text(encoding="utf-8"))
    parts = [p for p in _split_payload(region) if p]

    quoted = [_quote_payload_part(p) for p in parts]
    out_path.write_text(",\n".join(quoted) + "\n", encoding="utf-8")
    print(f"wrote {out_path} ({len(parts)} segments)")


if __name__ == "__main__":
    main()
