# reinforcement-learning-pipeline

> **Publisher → bus → subscriber** layout for RL rollouts: each
> environment transition is a message you can log, buffer for replay, or
> fan out to trainers — same structural idea as a pub/sub data pipeline,
> in-process for tests and local runs.

```mermaid
flowchart LR
    ENV[("🌍<br/>envs/<br/>tiny_grid")]
    PUB["📤 publishers<br/>episode"]
    BUS{{"🚌 in-memory bus<br/><i>InMemoryBus</i>"}}
    SUB["📥 subscribers<br/>transition_log"]
    RUN["🎯 pipeline.runner"]
    OUT[/"📜 logs · 🧠 replay · 📈 metrics"/]

    ENV --> PUB --> BUS --> SUB --> OUT
    RUN -. wires .-> PUB
    RUN -. wires .-> BUS
    RUN -. wires .-> SUB

    classDef io fill:#0e1116,stroke:#2f81f7,stroke-width:1.5px,color:#e6edf3;
    classDef tool fill:#161b22,stroke:#3fb950,stroke-width:1.5px,color:#e6edf3;
    classDef brain fill:#161b22,stroke:#d29922,stroke-width:1.5px,color:#e6edf3;
    classDef out fill:#0e1116,stroke:#a371f7,stroke-width:1.5px,color:#e6edf3;
    class ENV io;
    class PUB,SUB tool;
    class BUS,RUN brain;
    class OUT out;
```

## Table of contents

- [Layout](#layout)
- [Architecture at a glance](#architecture-at-a-glance)
- [Quick start](#quick-start)
- [License](#license)

## Layout

- `src/rl_pipeline/bus/` — message bus
- `src/rl_pipeline/envs/` — tiny environments
- `src/rl_pipeline/publishers/` — episode / rollout publishers
- `src/rl_pipeline/subscribers/` — loggers, replay writers, metrics
- `src/rl_pipeline/pipeline/` — orchestration
- `datasets/` — optional offline-RL manifests

### Architecture at a glance

```mermaid
flowchart TB
    subgraph PIPE["🎯 pipeline · runner"]
        WIRE["wire env → publisher → bus → subscribers"]
    end
    subgraph ENVS["🌍 envs"]
        TG["tiny_grid.py"]
    end
    subgraph BUSBOX["🚌 bus"]
        PROTO["protocol.py · Transition"]
        MEM["in_memory.py · InMemoryBus"]
    end
    subgraph PUBS["📤 publishers"]
        EP["episode.py"]
    end
    subgraph SUBS["📥 subscribers"]
        TL["transition_log.py"]
    end
    WIRE --> TG --> EP --> MEM
    MEM --> TL
    PROTO -.types.-> MEM
    PROTO -.types.-> EP
    PROTO -.types.-> TL
```

## Quick start

```bash
cd reinforcement-learning-pipeline
python3 -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"
pytest
```

## License

MIT
