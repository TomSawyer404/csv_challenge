<div align="center">
<h1>CSV Challenge</h1>
</div>

通过对 Cargo 包管理工具和模块系统的学习，我们现在完全有能力写一个具有完整功能的包。以2017年 Cpp17 编码挑战赛为例，用 Rust 来实现挑战题。

这道挑战题是这样的：编写一个命令行工具，可以接收一个 CSV 文件，并且可以指定固定的值来覆盖指定列的所有数据，然后将结果输出到新的 CSV 文件。原始 CSV 文件内容如下所示：

```csv
First Name,Last Name,Age,City,Eyes color,Species
John,Doe,32,Tokyo,Blue,Human
Flip,Helm,12,Canberra,Red,Unknown
Terdos,Bendarian,165,Cracow,Blue,Magic tree
Dominik,Elpos,33,Paris,Purple,Orc
Brad,Doe,42,Dublin,Blue,Human
Ewan,Grath,51,New Delhi,Green,Human
```

然后执行命令如下：

```bash
$ ./your_program input/challenges.csv City Beijing output/output.csv
```

`your_program`为编译后的可执行程序，其接收三个参数，分别是字段名（`City`）、要替代的城市（`Beijing`）和输出的文件名（`output/output.csv`）。执行命令后，输出文件内容如下所示：

```csv
First Name,Last Name,Age,City,Eyes color,Species
John,Doe,32,Beijing,Blue,Human
Flip,Helm,12,Beijing,Red,Unknown
Terdos,Bendarian,165,Beijing,Blue,Magic tree
Dominik,Elpos,33,Beijing,Purple,Orc
Brad,Doe,42,Beijing,Blue,Human
Ewan,Grath,51,Beijing,Green,Human
```

可以看到，`City`字段下面的所有值都被替换成了`Beijing`。挑战题其实很简单，本意是想让参与者完全用 Cpp17 的新特性来完成。借用此题，现在用 Rust 来实现。

本项目参考自[《Rust编程之道》第 10.3 节](http://product.dangdang.com/26475568.html)

---
