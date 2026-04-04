# Yongle

Yongle 是一个面向结构化学习与知识处理的系统。
它的长期方向不是“只是一个笔记应用”，也不是“只是一个 SRS 应用”，而是把结构化知识编写、确定性的渲染与提取、间隔重复、增量阅读、工作流与状态机驱动的学习过程，以及可复用的非 GUI 核心整合到同一个体系中。

## 项目定位

Yongle 以“内容为源、派生视图为结果”为基本方向，强调：

- 结构化知识，而不是松散文本堆积
- 可检查、可复现的处理过程，而不是不透明的魔法行为
- 同一份源内容向多种学习投影演化，而不是每种功能各自维护孤立数据
- 可复用核心优先，GUI 和外部集成随后展开

## 长期方向

项目预期会逐步覆盖以下能力：

- structured knowledge authoring
- deterministic rendering and extraction
- spaced repetition
- incremental reading
- workflow / state-machine based learning processes
- a reusable non-GUI core
- multiple frontends and tools built around the core

从架构上看，代码库预计会朝以下分层演化：

1. 核心领域模型与共享抽象
2. 解析、编译、提取与查询逻辑
3. 学习流水线与调度
4. 可复用的 CLI、服务与应用接口
5. GUI 与外部集成层

## 当前工作区概览

当前仓库是一个 Rust workspace，由多个 `yongle-*` crate 组成。
其中 `yongle-core` 是面向集成的顶层核心库，负责连接其他底层 crate，而不是承担无边界的“杂项容器”角色。

按职责可以做一个简要分组：

- 核心整合：`yongle-core`
- 标识与原语：`yongle-id`、`yongle-primitives`
- 摘要与存储：`yongle-digest`、`yongle-cas`、`yongle-cas-types`、`yongle-cas-meta`
- 其他支撑 crate：`yongle-manifest`、`yongle-gc`
- CLI：`ylctl`

这些 crate 目前仍处于早期演进阶段，边界会继续收敛，但整体方向已经明确：尽量把可复用逻辑沉淀在清晰、聚焦的 crate 中，避免把 GUI 或单一前端需求提前压进核心层。

## 开发与文档约定

项目级说明文档优先使用中文，例如 `README`、架构说明、设计取舍记录、开发日志、roadmap 以及临时 RFC 或草案。

代码相关文档继续保持英文，包括：

- 所有代码注释
- 所有 `///` doc comments
- crate-level docs
- 错误类型说明
- trait contract / safety / invariant 说明

Rust 社区惯例使用英文的文件或字段不在本仓库文档中文化范围内，例如 `Cargo.toml` 中的 crate 描述与发布元数据。

## 当前状态

项目目前处于早期阶段，核心方向和架构目标已经建立，但大量具体子系统仍在演进中。
当前 `README` 只提供基础入口与方向说明，不承担完整设计文档、架构白皮书或实现细节索引的职能。
