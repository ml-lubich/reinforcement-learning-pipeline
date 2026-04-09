# reinforcement-learning-pipeline

**Publisher → bus → subscriber** layout for RL rollouts: each environment transition is a message you can log, buffer for replay, or fan out to trainers—same structural idea as a pub/sub data pipeline, in-process for tests and local runs.

## Layout

- `src/rl_pipeline/bus/` — message bus
- `src/rl_pipeline/envs/` — tiny environments
- `src/rl_pipeline/publishers/` — episode / rollout publishers
- `src/rl_pipeline/subscribers/` — loggers, replay writers, metrics
- `src/rl_pipeline/pipeline/` — orchestration
- `datasets/` — optional offline-RL manifests

## Quick start

```bash
cd reinforcement-learning-pipeline
python3 -m venv .venv && source .venv/bin/activate
pip install -e ".[dev]"
pytest
```

## License

MIT
