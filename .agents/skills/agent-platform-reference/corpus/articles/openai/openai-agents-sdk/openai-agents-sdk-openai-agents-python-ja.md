# OpenAI Agents SDK

[OpenAI Agents SDK](https://github.com/openai/openai-agents-python) を使用すると、抽象化を最小限に抑えた軽量で使いやすいパッケージで、エージェント型 AI アプリを構築できます。これは、以前のエージェント向け実験プロジェクトである [Swarm](https://github.com/openai/swarm/tree/main) を本番環境向けにアップグレードしたものです。Agents SDK は、非常に少数の基本コンポーネントで構成されています。

* **エージェント**: 指示とツールを備えた LLM
* **Agents as tools / ハンドオフ**: エージェントが特定のタスクを別のエージェントに委任できる仕組み
* **ガードレール**: エージェントの入力と出力を検証できる仕組み

これらの基本コンポーネントを Python と組み合わせることで、ツールとエージェント間の複雑な関係を表現し、学習コストを抑えながら実用的なアプリケーションを構築できます。さらに、SDK には組み込みの **トレーシング** が含まれており、エージェント型フローの可視化とデバッグに加え、評価やアプリケーション向けモデルのファインチューニングも行えます。

## Agents SDK を使用する理由

SDK には、設計を支える 2 つの原則があります。

1. 使用する価値があるだけの機能を備えつつ、すぐに習得できるよう基本コンポーネントを十分に少なくすること。
2. 初期設定のままでも適切に動作しながら、実際の処理を詳細にカスタマイズできること。

SDK の主な機能は次のとおりです。

* **エージェント**: 指示、ツール、ガードレール、ハンドオフ、およびタスクが完了するまで継続する組み込みループを備えたエージェントを構築できます。
* **サンドボックスエージェント**: 実際の隔離されたワークスペース内で専門エージェントを実行できます。サンドボックスエージェントは、マニフェストで定義されたファイル、サンドボックスクライアントの選択、再開可能なサンドボックスセッションをサポートします。
* **Realtime エージェント**: `gpt-realtime-2.1`、自動中断検出、コンテキスト管理、ガードレールなどを使用して、強力な音声エージェントを構築できます。
* **音声エージェント**: 音声テキスト変換、エージェントワークフロー、テキスト音声変換を組み合わせた音声パイプラインを構築できます。
* **Python ファースト**: 新しい抽象化を学ぶ必要はなく、組み込みの言語機能を使用してエージェントをオーケストレーションし、連鎖させることができます。
* **Agents as tools / ハンドオフ**: 複数のエージェント間で作業を調整し、委任するための強力な仕組みです。
* **ガードレール**: エージェントの実行と並行して入力検証と安全性チェックを実行し、チェックに合格しない場合は即座に失敗させます。
* **関数ツール**: スキーマの自動生成と Pydantic を利用した検証により、任意の Python 関数をツールに変換できます。
* **MCP サーバーツール呼び出し**: リモートの MCP ツールを関数ツールとともにエージェントへ公開するための組み込み統合です。
* **セッション**: エージェントループ内で作業コンテキストを維持するための永続的なメモリレイヤーです。
* **Human in the loop**: エージェントの実行中に人間を関与させるための組み込みの仕組みです。
* **トレーシング**: ワークフローを可視化、デバッグ、監視するための組み込みのトレーシングです。OpenAI の評価、ファインチューニング、蒸留ツール群をサポートしています。

## Agents SDK と Responses API の選択

SDK は、OpenAI モデルに対してデフォルトで Responses API を使用しますが、モデル呼び出しをより高レベルのランタイムでラップします。

次の場合は、Responses API を直接使用します。

* ループ、ツールのディスパッチ、状態管理を自分で制御したい場合
* ワークフローの実行時間が短く、主な目的がモデルの応答を返すことである場合

次の場合は、Agents SDK を使用します。

* ターン、ツール実行、ガードレール、ハンドオフ、またはセッションをランタイムに管理させたい場合
* エージェントが成果物を生成する場合や、連携された複数のステップにわたって動作する必要がある場合
* [サンドボックスエージェント](https://openai.github.io/openai-agents-python/ja/sandbox_agents/)を通じて、実際のワークスペースまたは再開可能な実行が必要な場合

アプリケーション全体で、どちらか一方だけを選択する必要はありません。多くのアプリケーションでは、管理されたワークフローに SDK を使用し、より低レベルの処理では Responses API を直接呼び出します。

## インストール

```
pip install openai-agents
```

## Hello world の例

```
from agents import Agent, Runner

agent = Agent(name="Assistant", instructions="You are a helpful assistant")

result = Runner.run_sync(agent, "Write a haiku about recursion in programming.")
print(result.final_output)

# Code within the code,
# Functions calling themselves,
# Infinite loop's dance.
```

(*これを実行する場合は、環境変数 `OPENAI_API_KEY` を設定してください*)

```
export OPENAI_API_KEY=sk-...
```

## はじめに

* [クイックスタート](https://openai.github.io/openai-agents-python/ja/quickstart/)で、最初のテキストベースのエージェントを構築します。
* 次に、[エージェントの実行](https://openai.github.io/openai-agents-python/ja/running_agents/#choose-a-memory-strategy)で、ターン間で状態を引き継ぐ方法を決定します。
* タスクが実際のファイル、リポジトリ、またはエージェントごとに隔離されたワークスペースの状態に依存する場合は、[サンドボックスエージェントのクイックスタート](https://openai.github.io/openai-agents-python/ja/sandbox_agents/)を参照してください。
* ハンドオフとマネージャー型オーケストレーションのどちらを使用するか決める場合は、[エージェントオーケストレーション](https://openai.github.io/openai-agents-python/ja/multi_agent/)を参照してください。

## 目的別ガイド

実行したい作業は決まっていても、説明がどのページにあるか分からない場合は、次の表を使用してください。

| 目的 | 最初に参照するページ |
| --- | --- |
| 最初のテキストエージェントを構築し、一連の実行全体を確認する | [クイックスタート](https://openai.github.io/openai-agents-python/ja/quickstart/) |
| 関数ツール、ホスト型ツール、または Agents as tools を追加する | [ツール](https://openai.github.io/openai-agents-python/ja/tools/) |
| 実際の隔離されたワークスペース内で、コーディング、レビュー、またはドキュメント処理を行うエージェントを実行する | [サンドボックスエージェントのクイックスタート](https://openai.github.io/openai-agents-python/ja/sandbox_agents/)と[サンドボックスクライアント](https://openai.github.io/openai-agents-python/ja/sandbox/clients/) |
| ハンドオフとマネージャー型オーケストレーションのどちらを使用するか決定する | [エージェントオーケストレーション](https://openai.github.io/openai-agents-python/ja/multi_agent/) |
| ターン間でメモリを保持する | [エージェントの実行](https://openai.github.io/openai-agents-python/ja/running_agents/#choose-a-memory-strategy)と[セッション](https://openai.github.io/openai-agents-python/ja/sessions/) |
| OpenAI モデル、WebSocket トランスポート、または OpenAI 以外のプロバイダーを使用する | [モデル](https://openai.github.io/openai-agents-python/ja/models/) |
| 出力、実行項目、中断、再開状態を確認する | [実行結果](https://openai.github.io/openai-agents-python/ja/results/) |
| `gpt-realtime-2.1` を使用して低レイテンシーの音声エージェントを構築する | [Realtime エージェントのクイックスタート](https://openai.github.io/openai-agents-python/ja/realtime/quickstart/)と[Realtime トランスポート](https://openai.github.io/openai-agents-python/ja/realtime/transport/) |
| 音声テキスト変換、エージェント、テキスト音声変換を組み合わせたパイプラインを構築する | [音声パイプラインのクイックスタート](https://openai.github.io/openai-agents-python/ja/voice/quickstart/) |
