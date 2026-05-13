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
- [Rollout sequence](#rollout-sequence)
- [Quick start](#quick-start)
- [License](#license)

## Rollout sequence

```mermaid
sequenceDiagram
    participant R as pipeline.runner
    participant E as tiny_grid env
    participant P as episode publisher
    participant B as InMemoryBus
    participant L as transition_log
    participant RB as replay buffer

    R->>L: subscribe(B, topic="transition")
    R->>RB: subscribe(B, topic="transition")
    R->>E: reset()
    E-->>R: state s0
    loop until done
        R->>P: act(s_t) -> a_t
        P->>E: step(a_t)
        E-->>P: (s_{t+1}, r, done)
        P->>B: publish(Transition{s, a, r, s'})
        B->>L: deliver
        B->>RB: deliver
    end
    R->>B: close()
```

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
