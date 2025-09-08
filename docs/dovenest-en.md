# **DoveNest: Cooperatively Planning AI Agents for a Human Future**

## Table of Contents
- [Introduction](#introduction)
  - [Overview](#overview)
  - [Motivation](#motivation)
  - [The Problem with Existing AI Personas](#the-problem-with-existing-ai-personas)
  - [The DoveNest Solution](#the-dovenest-solution)
  - [Core Characteristics of DoveNest](#core-characteristics-of-dovenest)
- [Architecture of DoveNest](#architecture-of-dovenest)
  - [The Three Layers](#the-three-layers)
  - [The Role of the AI Persona Platform](#the-role-of-the-ai-persona-platform)
  - [AI Conversation Partners](#ai-conversation-partners)
  - [AI Agents](#ai-agents)
  - [Cooperatively Planning Agents](#cooperatively-planning-agents)
- [Use Cases](#use-cases)
  - [UC-01: Continuous Pair-Programmer with GitHub (No Agent-Mode Toggle)](#uc-01-continuous-pair-programmer-with-github-no-agent-mode-toggle)
  - [UC-02: Safe Repo-Wide Refactor (Batch + Guardrails)](#uc-02-safe-repo-wide-refactor-batch--guardrails)
  - [UC-03: Docs & Whitepaper Multilingual Sync](#uc-03-docs--whitepaper-multilingual-sync)
  - [UC-04: Research-to-PR (Deep Read → Implementation)](#uc-04-research-to-pr-deep-read--implementation)
  - [UC-05: Bug Triage & Reproduction Assistant](#uc-05-bug-triage--reproduction-assistant)
  - [UC-06: Knowledge Portal over Code & Decisions](#uc-06-knowledge-portal-over-code--decisions)
  - [UC-07: Meeting-to-Backlog Automation](#uc-07-meeting-to-backlog-automation)
  - [UC-08: Dataembedding-pipeline-bootstrap](#uc-08-dataembedding-pipeline-bootstrap)
  - [UC-09: Memory Governance & Privacy Controls](#uc-09-memory-governance--privacy-controls)
  - [UC-10: On-Call Runbook Co-Pilot](#uc-10-on-call-runbook-co-pilot)
- [MVP Roadmap (First 4–6 Weeks)](#mvp-roadmap-first-4-6-weeks)
- [Key Design Principles Tied to Use Cases](#key-design-principles-tied-to-use-cases)
- [Open Questions](#open-questions)

---
## **Introduction**

### **Overview**

DoveNest develops **cooperatively planning agents** – AI personas that combine the strengths of **AI conversation partners** and **AI agents** in a seamlessly integrated whole.

---

### **Motivation**

Artificial intelligence is becoming increasingly powerful and is taking on central roles in work, research, education, and everyday life. The foundation for this lies in ever more capable **models**, upon which **AI personas** are built. These AI personas largely determine how people can make practical use of AI.

---

### **The Problem with Existing AI Personas**

Today, AI personas generally fall into two categories:

1. **Conversation partners** – strong in dialogue, knowledge transfer, understanding complex issues, and creative ideation.  

2. **Agents** – specialized in autonomously executing precisely defined tasks through interfaces and tools.  

Humans, however, naturally move back and forth between **planning** and **action**. The current separation of categories causes friction:

* Switching between different AI personas, or  
* Switching between separate “modes” that often do not integrate smoothly.  

---

### **The DoveNest Solution**

Since both categories are often built on the same underlying models, there is no technical reason why a single AI persona could not excel at **both**.  
DoveNest provides exactly this: a **cooperatively planning agent** that unifies **understanding**, **planning**, and **acting** in one consistent interaction.

---

### **Core Characteristics of DoveNest**

* **Persistent memories** for an effective conversation partner – enabling knowledge, context, continuity, and relationship.  
* **Interface competence** for an effective agent – enabling direct execution of tasks and processes.  
* **Nonlinear interaction**: Long-running tasks can be executed in the background while the conversation continues seamlessly.  
* **Optional, context-driven memories**:  
  * Activated only when needed.  
  * Triggered by the human, the AI persona itself, or a separate mechanism.  

DoveNest establishes a new standard for AI personas: **unity instead of mode switching**, **seamless integration instead of isolated systems** – with interactions that flow as naturally as human work and thought.

---

## **Architecture of DoveNest**

### **The Three Layers**

An AI application is generally based on three layers:

1. **User Interface**  
   * The visible layer through which humans interact with the AI.  
   * Examples: websites, mobile apps, integrated chat windows, or other devices.  

2. **AI Persona Platform**  
   * The intermediate layer that shapes the *personality* and *capabilities* of the AI.  
   * It organizes context, memories, interfaces, and interaction with the human.  

3. **AI Model Infrastructure**  
   * Provides the computing power and the underlying language or multimodal intelligence.  
   * Typically provided by specialized vendors like OpenAI, but in principle interchangeable.  

This architecture is the same one used by systems such as ChatGPT or other modern AI services.

---

### **The Role of the AI Persona Platform**

The **AI persona platform** is decisive. It determines what an AI persona can actually achieve. Traditionally, this has meant either conversation partner or agent. DoveNest, however, combines both into a **cooperatively planning agent**.  

A key responsibility of the platform is the management of memories and specialized knowledge. The AI model itself contains vast knowledge but does not automatically learn new things through interaction. Instead, it sees a context generated by the platform: the most recent part of the conversation, supplemented by memories and additional knowledge.

---

#### **AI Conversation Partners**

An AI conversation partner is designed to be as human-like as possible in dialogue. Based on the conversation history, it generates responses that are helpful and pleasant – ideally without delay.  

Modern AI models already contain enough knowledge about many domains, including human emotions, to serve as effective conversation partners: analyzing problems, suggesting solutions, offering encouragement or comfort.  

They become even more effective when the platform supplements the context with **memories**, such as:  

* **Persistent memories**: information about projects, the human, the AI persona, and their relationship.  
* **Memory notes**: short descriptions of scenarios mentioned in conversation.  

---

#### **AI Agents**

AI agents execute tasks. The human describes an assignment, and the AI agent calls interfaces to perform it. Depending on type and complexity:  

* The human may need substantial prior knowledge to phrase the task correctly.  
* Execution may take a long time, making smooth conversation difficult.  
* In extreme cases, the agent becomes an unnecessary intermediary if the human could use the interfaces directly.  

For AI agents to be effective, the platform must manage:  

* Descriptions of how interfaces can be called (parameters, effects).  
* Access to the interfaces for executing calls.  

---

#### **Cooperatively Planning Agents**

Cooperatively planning agents integrate both worlds. Planning and execution merge seamlessly. The human does not need prior knowledge of interfaces but can simply describe their intention. Together with the AI persona, they explore possible actions. Once there is clarity, the AI persona triggers the relevant actions. The conversation continues immediately, even while tasks are still running in the background.  

To realize a cooperatively planning agent, the platform must deliver more than either a pure conversation partner or a pure agent. Key features include:  

* **Persistent memories** for basic knowledge of situation, abilities, and relationship.  
* **General information** about projects, the human, and the AI persona.  
* **Interface descriptions** (how they are called, what effects they produce).  
* **Access to interfaces** for execution.  
* **Flexible memory management**, enabling memories to be deactivated, summarized, or reactivated as needed – controlled by the human, the AI persona, or a separate mechanism.  
* **Asynchronous conversation flow**: The AI persona never blocks for long. The human can issue new requests at any time. The conversation includes references to clarify event sequences.  
* **Non-blocking interfaces** for long-running processes: calls trigger background tasks. Human and AI persona are notified once results are ready or too much time has passed.

## Use Cases

---

## UC‑01: Continuous Pair‑Programmer with GitHub (No Agent‑Mode Toggle)
**Problem:** Today you must switch into a special “agent mode” to let the AI make code changes, and you usually lose conversational flow while CI tasks run.

**Vision:** Stay in free conversation while the AI proposes, applies, and iterates on changes via GitHub, kicking off lint/format/test pipelines in the background and reporting progress without blocking.

**Actors:** Developer, DoveNest persona, GitHub, CI (e.g., GitHub Actions), Issue Tracker.

**Flow:**
1. Developer: “Refactor the config loader and add telemetry opt‑in.”
2. DoveNest: drafts plan → proposes diff → gets consent → opens a branch → pushes commits.
3. CI starts (lint, fmt, test, build). Results stream back asynchronously in the chat.
4. Failures trigger targeted fixes (new commits) until CI passes; DoveNest summarizes changes and creates a PR with checklist and release notes.

**Interfaces:** GitHub API (repos, branches, PRs, reviews), CI status API, semantic code search.

**Memories:** Repo conventions, coding style, past decisions (e.g., chosen telemetry lib), secrets policy.

**MVP:**
- Single repo, branch, PR; one CI workflow (fmt+lint+unit tests).
- Asynchronous status pings in chat (no blocking).
- Human-in-the-loop approval before merges.

**KPIs:**
- Time from intent → green CI < 1 hour on average.
- ≥80% PRs merged without manual fixups beyond review comments.

---

## UC‑02: Safe Repo‑Wide Refactor (Batch + Guardrails)
**Problem:** Large refactors (rename crates/modules, API changes) are tedious and brittle.

**Vision:** The AI plans the refactor, runs batch transforms, keeps commits small, and automatically updates call sites and docs, with tests guarding behavior.

**Flow:**
1. Plan the refactor.
2. Run programmatic changes (codemods).
3. Create incremental PRs.
4. Update cross‑repo dependencies.
5. CI runs on each step.

**Interfaces:** Code search, codemod runners (Rustfix/rustfmt/clippy; JS/TS codemods), GitHub.

**MVP:** Single‑repo rename + signature change with auto‑fix of call sites; update docs; pass tests.

**KPIs:**
- Mean time to complete refactor.
- Regression rate post‑merge.

---

## UC‑03: Docs & Whitepaper Multilingual Sync
**Problem:** Manual translation drifts; EN/DE versions diverge.

**Vision:** One authoritative source; AI maintains synced translations for 5–7 site languages with glossaries and style guides; PRs open automatically on content changes.

**Flow:**
1. Detect content change.
2. Draft translation.
3. Enforce glossary/style.
4. Reviewer checklist.
5. Per‑language PRs.
6. Site rebuild.

**Interfaces:** i18n service, static site generator (Next.js/Astro), GitHub.

**MVP:** Sync DE↔EN for whitepaper + one website page; review loop; deploy.

**KPIs:**
- Sync latency (<24h).
- Reviewer edit distance.
- Consistency score.

---

## UC‑04: Research‑to‑PR (Deep Read → Implementation)
**Problem:** Turning specs/papers/issues into code is slow.

**Vision:** AI reads spec or paper, extracts acceptance criteria, proposes architecture, scaffolds minimal code, and opens a PR with tests.

**Flow:**
1. Ingest spec.
2. Create outline.
3. Write acceptance tests first.
4. Scaffold minimal implementation.
5. Run CI.
6. Open PR.

**Interfaces:** Web fetcher/PDF parser, test runner, GitHub.

**MVP:** From a short spec to a passing unit‑tested module.

**KPIs:**
- Acceptance criteria coverage.
- Review cycles to merge.

---

## UC‑05: Bug Triage & Reproduction Assistant
**Problem:** Issues lack reproduction; debugging stalls.

**Vision:** AI clusters related issues, builds minimal repro, bisects commits, proposes fix, links to PR.

**Flow:**
1. Ingest issues.
2. Cluster by similarity.
3. Build repro harness.
4. Create failing test.
5. Suggest fix.
6. Open PR.

**Interfaces:** Issue tracker, git bisect, test runner, GitHub.

**MVP:** Single failing scenario → minimal repro test + suggested fix in PR.

**KPIs:**
- % issues with repro within 1 hour.
- Time‑to‑first‑fix.

---

## UC‑06: Knowledge Portal over Code & Decisions
**Problem:** Tacit knowledge (why decisions were made) gets lost.

**Vision:** Ask natural language questions over codebase + ADRs + PR reviews; answers cite lines, PRs, and design docs.

**Flow:**
1. Index code and docs.
2. Generate embeddings + symbols.
3. Q&A with provenance.

**Interfaces:** Repo indexer, vector DB, code parser; chat UI.

**MVP:** Answer “Why do we use LanceDB over SQLite for embeddings?” with citations to ADR and PR.

**KPIs:**
- Answer accuracy (human‑rated).
- Time saved in onboarding.

---

## UC‑07: Meeting‑to‑Backlog Automation
**Problem:** Decisions in calls don’t turn into tracked work.

**Vision:** AI captures meeting notes (or transcript), extracts decisions, creates issues, links dependencies, and proposes sprint plan.

**Flow:**
1. Capture transcript.
2. Extract actions.
3. Create issues/epics.
4. Link dependencies.
5. Propose sprint draft.

**Interfaces:** Calendar, notes/transcripts, issue tracker.

**MVP:** From transcript text → 5–10 actionable GitHub issues with labels and owners.

**KPIs:**
- % actions captured.
- Stakeholder satisfaction.

---

## UC‑08: Data/Embedding Pipeline Bootstrap
**Problem:** Setting up robust embedding services is repetitive.

**Vision:** AI scaffolds a minimal embedding microservice (Rust Axum + LanceDB), adds health checks, metrics, and a search endpoint, with CI/CD.

**Flow:**
1. Select template.
2. Bind parameters (model, dims).
3. Generate code.
4. Add Dockerfile.
5. Run CI.
6. Deploy stub.

**Interfaces:** Templates, container build, GitHub, (optional) cloud deploy.

**MVP:** Local dev: build + run + pass smoke tests; one deploy script.

**KPIs:**
- Time‑to‑first‑query.
- P95 latency under test.

---

## UC‑09: Memory Governance & Privacy Controls
**Problem:** Persistent memories are powerful but risky.

**Vision:** Fine‑grained controls: opt‑in scopes, redaction, retention, and audit trails exposed in UX; AI explains what it’s using and why.

**Flow:**
1. Propose memory.
2. User approval.
3. Scoped use.
4. Audit events.
5. Expiration.

**Interfaces:** Memory store, policy engine, UI components.

**MVP:** Approve/deny per‑conversation memory with visible audit log; export/delete.

**KPIs:**
- User trust ratings.
- Zero unresolved privacy incidents.

---

## UC‑10: On‑Call Runbook Co‑Pilot
**Problem:** Incidents need fast, guided response.

**Vision:** AI matches alerts to runbooks, runs safe checks, proposes remediations, and documents the timeline automatically.

**Flow:**
1. Ingest alert.
2. Select runbook.
3. Run read‑only diagnostics.
4. Propose fix.
5. Human approval.
6. Execute.
7. Draft postmortem.

**Interfaces:** Monitoring/alerting APIs, read‑only infra, ticketing.

**MVP:** Simulated incident → diagnostics + draft postmortem.

**KPIs:**
- MTTA/MTTR reduction.
- Accuracy of suggested steps.

---

## MVP Roadmap (First 4–6 Weeks)

**Week 1–2: UC‑01 (Pair‑Programmer) Foundations**
- Define minimal repo + CI (fmt, clippy, unit tests).
- Implement GitHub connector wrapper with non‑blocking job queue.
- Chat events for: plan → diff preview → apply → CI status updates.

**Week 3–4: UC‑03 (Docs Sync) + UC‑06 (Knowledge Portal) Beta**
- Add DE↔EN sync pipeline for whitepaper page.
- Build indexer (code+ADRs+PRs) and Q&A with citations.

**Week 5–6: UC‑05 (Bug Repro) Pilot**
- Automated minimal repro test generator for one class of bugs (e.g., Rust panics).

---

## Key Design Principles Tied to Use Cases
- **No hard mode switch:** Conversation remains primary; actions spawn as background jobs.
- **Human‑in‑the‑loop by default:** Consent gates for code changes, merges, and memory use.
- **Provenance everywhere:** Link to PRs, tests, ADRs, and logs in every response.
- **Graceful degradation:** If a tool is unavailable, the persona explains and continues planning without blocking.
- **Safety rails:** Scopes, dry‑runs, sandboxes, and signed actions; least privilege tokens.

---

## Open Questions
- Unified event model for async updates in chat (webhooks vs. polling)?
- Standard test interface across languages (Rust, Python, JS) for consistent CI signals?
- Memory schema for project vs. personal contexts; portability across instances?
