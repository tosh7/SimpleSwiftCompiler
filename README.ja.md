# Simple Swift Compiler

Rustで実装されたシンプルなSwiftコンパイラです。このプロジェクトはSwiftプログラミング言語のサブセットに対して、レキサー、パーサー、LLVMコードジェネレータを実装しています。

## 機能

- **レキサー**: Swiftソースコードをトークン化
- **パーサー**: 演算子の優先順位を考慮した抽象構文木（AST）の構築
- **LLVMコードジェネレータ**: ASTからLLVM IRを生成
- **ネイティブ実行**: LLVMツールチェーンを使用したコードのコンパイルと実行
- **算術演算**: 基本的な四則演算（+、-、*、/）をサポート
- **変数宣言**: `let`による変数宣言（型推論付き）
- **変数参照**: 式内での宣言済み変数の使用
- **コマンドラインオプション**: 詳細なコンパイル出力のためのverboseモード

## プロジェクト構成

```
simple-swift-compiler/
├── src/
│   ├── main.rs          # エントリポイント、CLI引数解析
│   ├── compiler.rs      # メインコンパイラオーケストレータ
│   ├── lexer.rs         # 字句解析器
│   ├── token.rs         # トークン定義
│   ├── parser.rs        # パーサー実装
│   ├── ast.rs           # ASTノード定義
│   ├── codegen.rs       # LLVM IRコード生成
│   ├── llvm_backend.rs  # LLVMバックエンド実装
│   └── options.rs       # コンパイラオプションとフラグ
├── example/
│   └── Test.swift       # サンプルSwiftコード
├── target/llvm/         # 生成されたLLVM IRファイル
│   ├── output.ll        # LLVM IR出力
│   ├── output.s         # アセンブリ出力（オプション）
│   └── output           # 実行可能ファイル（オプション）
└── Cargo.toml           # プロジェクト設定
```

## ビルド方法

```bash
cargo build
```

## コンパイラの実行

### 基本的な使い方

```bash
cargo run -- example/Test.swift
```

### Verboseモード

トークン、AST、LLVM IRを含む詳細なコンパイル出力を表示：

```bash
cargo run -- --verbose example/Test.swift
# または
cargo run -- -v example/Test.swift
```

### コマンドラインオプション

- `--verbose` または `-v`: すべてのコンパイル段階を表示するverbose出力を有効化

実行時の処理内容：
1. Swiftソースファイルの読み込み
2. 入力のトークン化（フロントエンド：字句解析）
3. トークンからASTへの解析（フロントエンド：構文解析）
4. LLVM IRコードの生成（フロントエンド：IR生成）
5. LLVM IRを`target/llvm/output.ll`に保存
6. LLVMツールチェーンを使用したコードの実行（インストール済みの場合）

### 実行例

入力ファイル `example/Test.swift`:
```swift
print(42)
print(42+2)
print(42-2)
print(42*2)
print(42/2)
```

#### デフォルト出力（verboseなし）:
```bash
$ cargo run -- example/Test.swift
Source code:
print(42)
print(42+2)
print(42-2)
print(42*2)
print(42/2)
42
44
40
84
21
```

#### Verbose出力:
```bash
$ cargo run -- --verbose example/Test.swift
Source code:
print(42)
print(42+2)
print(42-2)
print(42*2)
print(42/2)
Tokens:
0: Token { token_type: Print, lexeme: "print" }
1: Token { token_type: LeftParen, lexeme: "(" }
2: Token { token_type: Number, lexeme: "42" }
...

=== AST ===
Program([
    Print(Number(42)),
    Print(Binary { left: Number(42), operator: Add, right: Number(2) }),
    Print(Binary { left: Number(42), operator: Subtract, right: Number(2) }),
    Print(Binary { left: Number(42), operator: Multiply, right: Number(2) }),
    Print(Binary { left: Number(42), operator: Divide, right: Number(2) }),
])

=== LLVM IR ===
; ModuleID = 'swift_module'
source_filename = "swift_source"

declare i32 @printf(i8*, ...)

@.str = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

define i32 @main() {
entry:
  %1 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 42)
  %2 = add i32 42, 2
  %3 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %2)
  %4 = sub i32 42, 2
  %5 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %4)
  %6 = mul i32 42, 2
  %7 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %6)
  %8 = sdiv i32 42, 2
  %9 = call i32 (i8*, ...) @printf(i8* getelementptr inbounds ([4 x i8], [4 x i8]* @.str, i32 0, i32 0), i32 %8)
  ret i32 0
}

=== LLVM IR saved to target/llvm/output.ll ===

=== LLVM Execution ===
Execution result: 42
44
40
84
21
```

## サポートされている機能

現在、コンパイラは以下をサポートしています：
- `print`文
- 整数リテラル
- 算術式（+、-、*、/）
- 演算子の優先順位を考慮した式の解析
- 抽象構文木（AST）の生成

## 必要な環境

- Rust 1.56以降
- Cargo
- LLVMツールチェーン（オプション、コード実行用）

### macOSでのLLVMインストール

```bash
brew install llvm
export PATH="/opt/homebrew/opt/llvm/bin:$PATH"
```

### LinuxでのLLVMインストール

```bash
sudo apt-get install llvm  # Ubuntu/Debian
# または
sudo yum install llvm      # RHEL/CentOS
```

## 開発

ビルドせずにコンパイルエラーをチェック：

```bash
cargo check
```

## アーキテクチャ

コンパイラはモジュラーアーキテクチャに従っています：

1. **フロントエンド（ソース → LLVM IR）**：
   - `lexer.rs`: ソースコードをトークンに変換
   - `parser.rs`: 演算子の優先順位を考慮してトークンからASTを構築
   - `codegen.rs`: ASTからLLVM IRを生成

2. **バックエンド（LLVM IR → 実行）**：
   - `llvm_backend.rs`: LLVMツールチェーンとの連携を処理
   - ファイルへのIR保存
   - LLVMインタープリタ（lli）による実行
   - ネイティブコードへのコンパイル（llc + clang）

3. **オーケストレーション**：
   - `compiler.rs`: コンパイルパイプラインの調整
   - `main.rs`: CLIインターフェース

## 生成されるファイル

コンパイラを実行すると以下のファイルが生成されます：
- `target/llvm/output.ll` - LLVM IRコード
- `target/llvm/output.s` - アセンブリコード（llc使用時）
- `target/llvm/output` - ネイティブ実行可能ファイル（clang使用時）

## ライセンス

Apache 2.0