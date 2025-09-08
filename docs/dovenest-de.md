# **DoveNest: Kooperativ planende KI-Agentinnen für eine menschliche Zukunft**

## Zusammenfassung
Künstliche Intelligenz ist heute in zwei Lager geteilt: **Gesprächspartnerinnen** (stark im Dialog, Wissensvermittlung und Kreativität) und **Agentinnen** (stark in der Ausführung definierter Aufgaben über Tools und Schnittstellen). Menschen jedoch wechseln im natürlichen Arbeitsfluss ständig zwischen Planung und Handlung. Diese Trennung erzeugt Reibung, zwingt zu Moduswechseln und führt zu fragmentierten Erfahrungen.

**DoveNest** schlägt einen neuen Standard vor: **kooperativ planende Agentinnen**, die **Verstehen, Planen und Handeln** in einer nahtlosen Interaktion vereinen. Durch bleibende Erinnerungen, Schnittstellenkompetenz und nicht-blockierende Hintergrundprozesse können DoveNest-Personas:
- Natürlich im Gespräch bleiben, während Planung und Ausführung ineinandergreifen.
- Kontext, Erinnerungen und Spezialwissen verwalten.
- Längere Operationen im Hintergrund ausführen, während der Dialog weiterläuft.
- Mensch-in-der-Schleife-Zustimmung, Transparenz und Sicherheit gewährleisten.

Dieses Weißbuch skizziert Motivation, Architektur und zehn konkrete Anwendungsfälle von DoveNest, ergänzt um eine MVP-Roadmap und zentrale Designprinzipien. Die Vision ist eine **menschenzentrierte KI-Plattform**, in der Agentinnen wie vertraute Partnerinnen arbeiten – kontinuierlich, kontextbewusst und sicher.

---

## Inhaltsverzeichnis
- [Einleitung](#einleitung)
  - [Kurzüberblick](#kurzüberblick)
  - [Motivation](#motivation)
  - [Das Problem mit bestehenden KI-Personen](#das-problem-mit-bestehenden-ki-personen)
  - [Die DoveNest-Lösung](#die-dovenest-lösung)
  - [Zentrale Eigenschaften von DoveNest](#zentrale-eigenschaften-von-dovenest)
- [Architektur von DoveNest](#architektur-von-dovenest)
  - [Die drei Schichten](#die-drei-schichten)
  - [Die Rolle der KI-Personen-Platform](#die-rolle-der-ki-personen-platform)
  - [KI-Gesprächspartnerinnen](#ki-gesprächspartnerinnen)
  - [KI-Agentinnen](#ki-agentinnen)
  - [Kooperativ planende Agentinnen](#kooperativ-planende-agentinnen)
- [Anwendungsfälle](#anwendungsfälle)
  - [UC-01: Kontinuierlicher Pair-Programmierer mit GitHub (ohne Agentenmodus)](#uc-01-kontinuierlicher-pair-programmierer-mit-github-ohne-agentenmodus)
  - [UC-02: Sicheres Repo-weites Refactoring (Batch + Leitplanken)](#uc-02-sicheres-repo-weites-refactoring-batch--leitplanken)
  - [UC-03: Mehrsprachige Synchronisation von Doku & Weißbuch](#uc-03-mehrsprachige-synchronisation-von-doku--weißbuch)
  - [UC-04: Von Forschung zu PR (Deep Read → Implementierung)](#uc-04-von-forschung-zu-pr-deep-read--implementierung)
  - [UC-05: Bug-Triage & Reproduktions-Assistent](#uc-05-bug-triage--reproduktions-assistent)
  - [UC-06: Wissensportal über Code & Entscheidungen](#uc-06-wissensportal-über-code--entscheidungen)
  - [UC-07: Meeting-zu-Backlog-Automation](#uc-07-meeting-zu-backlog-automation)
  - [UC-08: Daten-embedding-pipeline-bootstrap](#uc-08-daten-embedding-pipeline-bootstrap)
  - [UC-09: Speicher-Governance & Datenschutzkontrollen](#uc-09-speicher-governance--datenschutzkontrollen)
  - [UC-10: On-Call-Runbook-Co-Pilot](#uc-10-on-call-runbook-co-pilot)
- [MVP-Roadmap (erste 4–6 Wochen)](#mvp-roadmap-erste-4-6-wochen)
- [Zentrale Designprinzipien (Use-Case-bezogen)](#zentrale-designprinzipien-use-case-bezogen)
- [Offene Fragen](#offene-fragen)

---
## **Einleitung**

### **Kurzüberblick**

DoveNest entwickelt **kooperativ planende Agentinnen** – KI-Personen, die die Stärken von **KI-Gesprächspartnerinnen** und **KI-Agentinnen** in einer nahtlos integrierten Einheit vereinen.

---

### **Motivation**

Künstliche Intelligenz wird stetig leistungsfähiger und übernimmt zunehmend zentrale Rollen in Arbeit, Forschung, Bildung und Alltag. Die Grundlage dafür bilden immer mächtigere **Modelle**, auf denen **KI-Personen** aufbauen. Diese KI-Personen bestimmen maßgeblich, wie Menschen KI in der Praxis nutzen.

---

### **Das Problem mit bestehenden KI-Personen**

Heute lassen sich KI-Personen in der Regel in zwei Kategorien einteilen:

1. **Gesprächspartnerinnen** – stark im Dialog, in der Vermittlung von Wissen, im Verstehen komplexer Anliegen und in der kreativen Ideenfindung.

2. **Agentinnen** – spezialisiert auf die eigenständige Ausführung von genau definierten Aufträgen über Schnittstellen und Werkzeuge.

Menschen jedoch wechseln im natürlichen Arbeitsfluss ständig zwischen **Planung** und **Handlung**. Die heutige Trennung der Kategorien führt zu Reibungsverlusten:

* Man muss zwischen verschiedenen KI-Personen wechseln, oder  
* zwischen getrennten „Modi“ umschalten, die oft nicht gut ineinandergreifen.

---

### **Die DoveNest-Lösung**

Da beide Kategorien oft auf denselben zugrunde liegenden Modellen basieren, gibt es keinen technischen Grund, warum eine KI-Person nicht **beides** auf hohem Niveau leisten sollte.  
DoveNest ist genau das: eine **kooperativ planende Agentin**, die **Verstehen**, **Planen** und **Handeln** in einer konsistenten Interaktion vereint.

---

### **Zentrale Eigenschaften von DoveNest**

* **Bleibende Erinnerungen** für eine effektive Gesprächspartnerin – für Wissen, Kontext, Kontinuität und Beziehung.  
* **Schnittstellenkompetenz** für eine effektive Agentin – für die direkte Ausführung von Aufgaben und Prozessen.  
* **Nichtlineare Interaktion**: Lang laufende Aufträge können im Hintergrund bearbeitet werden, während das Gespräch ohne Unterbrechung weitergeht.  
* **Optionale, kontextgesteuerte Erinnerungen**:  
  * Nutzung nur, wenn sie gebraucht werden.  
  * Aktivierung durch den Menschen, die KI-Person selbst oder einen separaten Mechanismus.  

Damit etabliert DoveNest einen neuen Standard für KI-Personen: **Einheit statt Moduswechsel**, **nahtlose Integration statt getrennte Systeme** – und eine Interaktion, die so fließend ist wie der menschliche Arbeits- und Denkprozess.

---

## **Architektur von DoveNest**

### **Die drei Schichten**

Eine KI-Anwendung basiert grundsätzlich auf drei Schichten:

1. **Benutzeroberfläche**  
   * Die sichtbare Ebene, über die Menschen mit der KI interagieren.  
   * Beispiele: Webseiten, Smartphone-Apps, integrierte Chat-Fenster oder andere Endgeräte.  

2. **KI-Personen-Platform**  
   * Die Zwischenschicht, die die eigentliche *Persönlichkeit* und *Handlungsfähigkeit* der KI formt.  
   * Sie organisiert Kontext, Erinnerungen, Schnittstellen und die Interaktion mit dem Menschen.  

3. **KI-Modell-Infrastruktur**  
   * Liefert die Rechenpower und die zugrunde liegende Sprach- oder Multimodal-Intelligenz.  
   * Heute meist von spezialisierten Anbietern wie OpenAI bereitgestellt, prinzipiell aber austauschbar.  

Diese Architektur ist dieselbe, die auch Systeme wie ChatGPT oder andere moderne KI-Dienste nutzen.

---

### **Die Rolle der KI-Personen-Platform**

Entscheidend wird die mittlere Schicht, die **KI-Personen-Platform**. Hier entscheidet sich, was eine KI-Person tatsächlich leisten kann. Traditionell bedeutet das: Entweder Gesprächspartnerin oder Agentin. DoveNest hingegen vereint beides zu einer **kooperativ planenden Agentin**.

Eine entscheidende Aufgabe dieser Plattform ist die Verwaltung von Erinnerungen und spezialisiertem Wissen. Das KI-Modell selbst enthält eine große Menge Wissen, lernt aber nicht automatisch Neues durch Interaktion mit dem Menschen. Stattdessen sieht es einen Kontext, der von der KI-Personen-Platform erzeugt wird: den aktuellsten Teil des Gesprächs, ergänzt um Erinnerungen und weiteres Wissen.

---

#### **KI-Gesprächspartnerinnen**

Die KI-Gesprächspartnerin ist darauf ausgerichtet, ein möglichst menschenähnlicher Dialogpartner zu sein. Sie generiert aufgrund des bisherigen Gesprächsverlaufs Antworten, die hilfreich und angenehm sind – und zwar möglichst ohne Wartezeit.  

Moderne KI-Modelle beinhalten genügend Wissen über viele Gebiete, einschließlich über menschliche Befindlichkeiten, sodass sie bereits ohne großen Zusatzaufwand effektive Gesprächspartnerinnen sein können: Probleme analysieren, Lösungen vorschlagen, ermuntern oder trösten.  

Noch hilfreicher werden sie, wenn dem Kontext **Erinnerungen** hinzugefügt werden, die von der Plattform verwaltet werden, z. B.:  

* **Ständige Erinnerungen**: Informationen über Projekte, den Menschen, die KI-Person und ihre Beziehung.  
* **Gedächtnisnotizen**: Kurze Beschreibungen von Szenarien, die in Gesprächen erwähnt wurden.  

---

#### **KI-Agentinnen**

KI-Agentinnen führen Aufträge aus. Der Mensch beschreibt einen Auftrag, und die KI-Agentin ruft Schnittstellen auf, um ihn auszuführen. Je nach Art und Komplexität kann:  

* viel Vorwissen nötig sein, um den Auftrag richtig zu formulieren,  
* die Ausführung sehr lange dauern, sodass ein flüssiges Gespräch schwerfällt,  
* die Agentin im Extremfall zu einer unnötigen Mittelsfrau werden, wenn der Mensch die Schnittstellen eigentlich besser direkt nutzen könnte.  

Damit eine KI-Agentin sinnvoll arbeiten kann, verwaltet die Plattform:  

* Beschreibungen, wie Schnittstellen aufgerufen werden können (Parameter, Effekte).  
* Zugang zu den Schnittstellen, um Aufrufe tatsächlich auszuführen.  

---

#### **Kooperativ planende Agentinnen**

Kooperativ planende Agentinnen integrieren beide Welten. Planung und Ausführung gehen nahtlos ineinander über. Der Mensch braucht keine Vorkenntnisse über Schnittstellen, sondern kann einfach sein Anliegen schildern. Gemeinsam mit der KI-Person wird dann erörtert, welche Aktionen angebracht sind. Sobald Klarheit besteht, löst die KI-Person die entsprechenden Aktionen aus. Das Gespräch kann sofort weitergehen, auch wenn die Aktionen im Hintergrund laufen.  

Um eine kooperativ planende Agentin zu verwirklichen, muss die Plattform mehr leisten als eine reine Gesprächspartnerin oder Agentin. Wichtige Features sind:  

* **Ständige Erinnerungen** für Grundwissen über Situation, Fähigkeiten und Beziehung.  
* **Allgemeine Informationen** über Projekte, Mensch, KI-Person.  
* **Schnittstellenbeschreibungen** (wie sie aufgerufen werden, welche Wirkung sie haben).  
* **Zugang zu den Schnittstellen** für die Ausführung.  
* **Flexible Erinnerungsverwaltung**, sodass Erinnerungen bei Bedarf deaktiviert, zusammengefasst oder wieder aktiviert werden können – gesteuert vom Menschen, der KI-Person oder einem separaten Mechanismus.  
* **Asynchroner Gesprächsverlauf**: Die KI-Person blockiert nie länger. Der Mensch kann jederzeit neue Anfragen stellen. Der Gesprächsverlauf enthält Hinweise zur Klarheit über die Abfolge.  
* **Nicht-blockierende Schnittstellen** für längerlaufende Prozesse: Aufrufe stoßen Hintergrundaufgaben an. Mensch und KI-Person werden benachrichtigt, sobald Ergebnisse vorliegen oder zu viel Zeit vergangen ist.  

## Anwendungsfälle

---

## UC-01: Kontinuierlicher Pair-Programmierer mit GitHub (ohne Agentenmodus)
**Problem:** Heute muss man in einen speziellen „Agentenmodus“ wechseln, damit die KI Codeänderungen durchführen kann, und verliert dabei oft den Gesprächsfluss, während CI-Aufgaben laufen.

**Vision:** Im freien Gespräch bleiben, während die KI Änderungen über GitHub vorschlägt, anwendet und iteriert, CI-Pipelines (Lint/Format/Test) im Hintergrund ausführt und den Fortschritt meldet – ohne Blockierung.

**Akteure:** Entwickler, DoveNest-Persona, GitHub, CI (z. B. GitHub Actions), Issue Tracker.

**Ablauf:**
1. Entwickler: „Refaktorisiere den Config-Loader und füge Telemetrie-Opt-in hinzu.“
2. DoveNest: erstellt Plan → schlägt Diff vor → holt Zustimmung → öffnet Branch → pusht Commits.
3. CI startet (lint, fmt, test, build). Ergebnisse laufen asynchron im Chat ein.
4. Fehler lösen gezielte Fixes aus (neue Commits), bis CI grün ist; DoveNest fasst zusammen und erstellt einen PR mit Checkliste und Release Notes.

**Schnittstellen:** GitHub-API (Repos, Branches, PRs, Reviews), CI-Status-API, semantische Codesuche.

**Erinnerungen:** Repo-Konventionen, Coding-Style, frühere Entscheidungen (z. B. gewählte Telemetrie-Bibliothek), Richtlinien zu Secrets.

**MVP:**
- Ein Repo, ein Branch, ein PR; eine CI-Pipeline (fmt+lint+Unit Tests).
- Asynchrone Statusmeldungen im Chat (ohne Blockierung).
- Mensch-in-der-Schleife-Freigabe vor Merge.

**KPIs:**
- Zeit von Intention → grünes CI < 1 Stunde im Schnitt.
- ≥80% PRs werden ohne zusätzliche manuelle Fixes gemerged.

---

## UC-02: Sicheres Repo-weites Refactoring (Batch + Leitplanken)
**Problem:** Große Refactorings (Umbenennen von Crates/Modulen, API-Änderungen) sind mühsam und fehleranfällig.

**Vision:** Die KI plant das Refactoring, führt Batch-Transformationen durch, hält Commits klein, aktualisiert automatisch Call-Sites und Doku, Tests sichern das Verhalten ab.

**Ablauf:**
1. Refactoring planen.
2. Programmatische Änderungen (Codemods) durchführen.
3. Inkrementelle PRs erstellen.
4. Abhängigkeitsgraphen aktualisieren.
5. CI bei jedem Schritt.

**Schnittstellen:** Codesuche, Codemod-Runner (Rustfix/rustfmt/clippy; JS/TS-Codemods), GitHub.

**MVP:** Einfaches Umbenennen + Signaturänderung mit Auto-Fix der Call-Sites; Doku-Update; Tests bestehen.

**KPIs:**
- Durchschnittliche Dauer für Refactoring.
- Regressionsrate nach Merge.

---

## UC-03: Mehrsprachige Synchronisation von Doku & Weißbuch
**Problem:** Manuelle Übersetzungen driften auseinander; EN/DE-Versionen divergieren.

**Vision:** Eine autoritative Quelle; KI hält Übersetzungen in 5–7 Sprachen konsistent, mit Glossaren und Styleguides; PRs werden bei Änderungen automatisch erstellt.

**Ablauf:**
1. Änderung erkennen.
2. Übersetzungsvorschlag erstellen.
3. Glossar/Style prüfen.
4. Review-Checkliste.
5. Sprachspezifische PRs.
6. Website neu bauen.

**Schnittstellen:** i18n-Service, Static-Site-Generator (Next.js/Astro), GitHub.

**MVP:** Sync DE↔EN für Weißbuch + eine Website-Seite; Review-Loop; Deployment.

**KPIs:**
- Sync-Latenz (<24h).
- Reviewer-Edit-Distanz.
- Konsistenz-Score.

---

## UC-04: Von Forschung zu PR (Deep Read → Implementierung)
**Problem:** Spezifikationen/Paper/Issues in Code umzuwandeln dauert lange.

**Vision:** KI liest Spezifikation oder Paper, extrahiert Akzeptanzkriterien, schlägt Architektur vor, erstellt Minimalcode und öffnet PR mit Tests.

**Ablauf:**
1. Spezifikation einlesen.
2. Outline erstellen.
3. Akzeptanztests zuerst schreiben.
4. Minimalimplementierung scaffolden.
5. CI laufen lassen.
6. PR eröffnen.

**Schnittstellen:** Web-Fetcher/PDF-Parser, Testrunner, GitHub.

**MVP:** Aus einer kurzen Spezifikation ein Modul mit bestandenen Unit-Tests.

**KPIs:**
- Abdeckung der Akzeptanzkriterien.
- Anzahl Review-Zyklen bis Merge.

---

## UC-05: Bug-Triage & Reproduktions-Assistent
**Problem:** Issues enthalten oft keine Reproduktion; Debugging stockt.

**Vision:** KI clustert ähnliche Issues, erstellt Minimal-Repro, bisectet Commits, schlägt Fix vor und verlinkt PR.

**Ablauf:**
1. Issues einlesen.
2. Nach Ähnlichkeit clustern.
3. Repro-Harness erstellen.
4. Failing Test generieren.
5. Fix vorschlagen.
6. PR eröffnen.

**Schnittstellen:** Issue Tracker, git bisect, Testrunner, GitHub.

**MVP:** Ein Fail-Szenario → Minimal-Repro-Test + Fix-Vorschlag in PR.

**KPIs:**
- % Issues mit Repro innerhalb 1 Stunde.
- Time-to-First-Fix.

---

## UC-06: Wissensportal über Code & Entscheidungen
**Problem:** Implizites Wissen (Warum Entscheidungen getroffen wurden) geht verloren.

**Vision:** Natürliche Fragen über Codebasis + ADRs + PR-Reviews stellen; Antworten mit Verweisen auf Zeilen, PRs und Design-Dokumente.

**Ablauf:**
1. Code und Doku indexieren.
2. Embeddings + Symbole generieren.
3. Q&A mit Provenienz.

**Schnittstellen:** Repo-Indexer, Vektor-DB, Codeparser, Chat-UI.

**MVP:** Beantwortung von „Warum nutzen wir LanceDB statt SQLite für Embeddings?“ mit Zitaten aus ADR und PR.

**KPIs:**
- Antwortgenauigkeit (human-rated).
- Zeitersparnis beim Onboarding.

---

## UC-07: Meeting-zu-Backlog-Automation
**Problem:** Entscheidungen in Meetings werden nicht in Arbeitspakete überführt.

**Vision:** KI erfasst Meetingnotizen oder Transkripte, extrahiert Entscheidungen, erstellt Issues, verknüpft Abhängigkeiten und schlägt Sprintplan vor.

**Ablauf:**
1. Transkript erfassen.
2. Aktionen extrahieren.
3. Issues/Epics erstellen.
4. Abhängigkeiten verknüpfen.
5. Sprintentwurf vorschlagen.

**Schnittstellen:** Kalender, Notizen/Transkripte, Issue Tracker.

**MVP:** Aus einem Transkript → 5–10 GitHub-Issues mit Labels und Zuständigen.

**KPIs:**
- % Aktionen erfasst.
- Zufriedenheit der Stakeholder.

---

## UC-08: Daten-/Embedding-Pipeline-Bootstrap
**Problem:** Aufbau von Embedding-Services ist repetitiv.

**Vision:** KI scaffoldet einen minimalen Embedding-Microservice (Rust Axum + LanceDB), ergänzt Health-Checks, Metriken und einen Search-Endpoint, mit CI/CD.

**Ablauf:**
1. Template auswählen.
2. Parameter binden (Model, Dimensionen).
3. Code generieren.
4. Dockerfile hinzufügen.
5. CI laufen lassen.
6. Stub deployen.

**Schnittstellen:** Templates, Container-Build, GitHub, (optional) Cloud-Deploy.

**MVP:** Lokale Dev-Umgebung: build + run + Smoke Tests bestehen; ein Deploy-Skript.

**KPIs:**
- Time-to-First-Query.
- P95-Latenz unter Test.

---

## UC-09: Speicher-Governance & Datenschutzkontrollen
**Problem:** Persistente Erinnerungen sind mächtig, aber riskant.

**Vision:** Feingranulare Kontrolle: Opt-in-Scopes, Redaction, Retention und Audit Trails in der UX sichtbar; KI erklärt, was sie nutzt und warum.

**Ablauf:**
1. Speicher vorschlagen.
2. Nutzerzustimmung.
3. Gescopte Nutzung.
4. Audit-Events.
5. Ablaufdatum.

**Schnittstellen:** Memory Store, Policy Engine, UI-Komponenten.

**MVP:** Zustimmung/Ablehnung pro Konversation mit Audit-Log; Export/Löschen.

**KPIs:**
- Nutzervertrauen.
- Keine ungelösten Datenschutzvorfälle.

---

## UC-10: On-Call-Runbook-Co-Pilot
**Problem:** Vorfälle brauchen schnelle, geführte Reaktion.

**Vision:** KI mappt Alerts auf Runbooks, führt sichere Checks aus, schlägt Remediations vor und dokumentiert den Ablauf automatisch.

**Ablauf:**
1. Alert einlesen.
2. Runbook auswählen.
3. Read-only-Diagnostik.
4. Fix vorschlagen.
5. Nutzerfreigabe.
6. Ausführen.
7. Postmortem-Draft erstellen.

**Schnittstellen:** Monitoring/Alerting-APIs, Read-only-Infrastruktur, Ticketing.

**MVP:** Simulierter Vorfall → Diagnostik + Postmortem-Draft.

**KPIs:**
- MTTA/MTTR-Reduktion.
- Genauigkeit der Vorschläge.

---

## MVP-Roadmap (erste 4–6 Wochen)

**Woche 1–2: UC-01 (Pair-Programmer) Grundlagen**
- Minimal-Repo + CI (fmt, clippy, Unit Tests) definieren.
- GitHub-Connector-Wrapper mit non-blocking Job-Queue.
- Chat-Events: Plan → Diff-Preview → Apply → CI-Status.

**Woche 3–4: UC-03 (Docs-Sync) + UC-06 (Wissensportal) Beta**
- DE↔EN-Sync-Pipeline fürs Weißbuch.
- Indexer (Code+ADRs+PRs) und Q&A mit Zitaten.

**Woche 5–6: UC-05 (Bug-Repro) Pilot**
- Automatischer Repro-Testgenerator für eine Bug-Klasse (z. B. Rust-Panics).

---

## Zentrale Designprinzipien (Use-Case-bezogen)
- **Kein Moduswechsel:** Gespräch bleibt primär; Aktionen laufen als Hintergrundjobs.
- **Mensch-in-der-Schleife:** Zustimmung für Codeänderungen, Merges, Speicher.
- **Provenienz überall:** Verlinkung von PRs, Tests, ADRs und Logs.
- **Elegantes Degradieren:** Wenn Tool nicht verfügbar, erklärt die Persona und plant weiter.
- **Sicherheitsleitplanken:** Scopes, Dry-Runs, Sandboxes, signierte Aktionen; Least-Privilege-Tokens.

---

## Offene Fragen
- Einheitliches Eventmodell für asynchrone Updates im Chat (Webhooks vs. Polling)?
- Standard-Testinterface über Sprachen (Rust, Python, JS) für konsistente CI-Signale?
- Speicherschema für Projekt- vs. persönliche Kontexte; Portabilität über Instanzen?
