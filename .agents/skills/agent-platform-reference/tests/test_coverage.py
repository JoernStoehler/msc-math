from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]


def read(filename):
    return (ROOT/'references'/filename).read_text().lower()


def test_offline_core_questions_have_local_coverage():
    cases={
        'chatgpt.md':['custom instructions','project instructions','memory','gpts','work','settings','plan'],
        'claude-chat.md':['instructions for claude','free','cowork','cloud','rollout'],
        'codex.md':['rust-v0.162.1','677,170','shallow','sparse','docs/skills.md'],
        'claude-code.md':['agents.md','claude.md','2.1.277','2.1.281','skills','/context'],
        'corrections.md':['github repository size','claude code never','project instructions','custom gpts'],
        'architecture.md':['chatgpt ordinary chat','codex cli','claude code','live-documentation','correction-miss'],
        'maintenance-workflows.md':['discover-claude','sync --jobs','etag','last-modified','benchmark_codex.sh'],
    }
    for file,keywords in cases.items():
        v=read(file)
        for keyword in keywords:
            assert keyword in v, (file,keyword)


def test_instructions_within_1500_char_limit():
    v=(ROOT/'references/custom-instructions.md').read_text()
    s=v.split('## Ready-to-paste instruction (under 1,500 characters)\n\n')[1].split('\n\n## Why')[0]
    assert 850<len(s)<=1500
    for word in ['completion','verify','blocked','simple','uncertainty']:
        assert word in s,word
