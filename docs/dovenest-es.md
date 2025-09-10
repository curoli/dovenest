# DoveNest: Agentes de IA de Planificación Cooperiva para un Futuro Humano

## Resumen Ejecutivo
Hoy en día, la inteligencia artificial está dividida entre **compañeros de conversación** (fuertes en diálogo, transferencia de conocimiento y creatividad) y **agentes** (fuertes en la ejecución de tareas mediante herramientas e interfaces). Los seres humanos, sin embargo, fluyen naturalmente entre la **planificación** y la **acción**. Esta separación genera fricción, obligando a cambiar de modo y produciendo experiencias fragmentadas.

**DoveNest** propone un nuevo estándar: **agentes de planificación cooperativa** que unifican **comprensión, planificación y acción** en una sola interacción fluida. Combinando memorias persistentes, competencia en interfaces y ejecución asíncrona no bloqueante, las personas DoveNest pueden:
- Mantener un diálogo natural mientras planifican y ejecutan tareas.  
- Gestionar contexto, memorias y conocimientos especializados.  
- Ejecutar operaciones largas en segundo plano mientras la conversación continúa.  
- Garantizar aprobación humana, trazabilidad y seguridad.  

Este libro blanco presenta la motivación, la arquitectura y diez casos de uso concretos de DoveNest, junto con una hoja de ruta de MVP y principios de diseño. La visión es una **plataforma de IA centrada en el ser humano**, donde los agentes actúan como colaboradores de confianza: continuos, contextuales y seguros.  

---

## Tabla de Contenidos
- [Introducción](#introducción)  
  - [Visión General](#visión-general)  
  - [Motivación](#motivación)  
  - [El Problema con las Personas de IA Existentes](#el-problema-con-las-personas-de-ia-existentes)  
  - [La Solución DoveNest](#la-solución-dovenest)  
  - [Características Clave de DoveNest](#características-clave-de-dovenest)  
- [Arquitectura de DoveNest](#arquitectura-de-dovenest)  
  - [Las Tres Capas](#las-tres-capas)  
  - [El Papel de la Plataforma de Personas de IA](#el-papel-de-la-plataforma-de-personas-de-ia)  
  - [Compañeros de Conversación de IA](#compañeros-de-conversación-de-ia)  
  - [Agentes de IA](#agentes-de-ia)  
  - [Agentes de Planificación Cooperativa](#agentes-de-planificación-cooperativa)  
- [Casos de Uso](#casos-de-uso)  
  - [UC-01: Programador en Pareja Continuo con GitHub (sin cambio de modo)](#uc-01-programador-en-pareja-continuo-con-github-sin-cambio-de-modo)  
  - [UC-02: Refactorización Segura a Escala de Repositorio (Lotes + Guardarraíles)](#uc-02-refactorización-segura-a-escala-de-repositorio-lotes--guardarraíles)  
  - [UC-03: Sincronización Multilingüe de Documentación y Libro Blanco](#uc-03-sincronización-multilingüe-de-documentación-y-libro-blanco)  
  - [UC-04: De la Investigación al PR (Lectura Profunda → Implementación)](#uc-04-de-la-investigación-al-pr-lectura-profunda--implementación)  
  - [UC-05: Asistente de Triage y Reproducción de Errores](#uc-05-asistente-de-triage-y-reproducción-de-errores)  
  - [UC-06: Portal de Conocimiento sobre Código y Decisiones](#uc-06-portal-de-conocimiento-sobre-código-y-decisiones)  
  - [UC-07: Automatización de Reunión a Backlog](#uc-07-automatización-de-reunión-a-backlog)  
  - [UC-08: Inicio de Pipeline de Datos/Embeddings](#uc-08-inicio-de-pipeline-de-datosembeddings)  
  - [UC-09: Gobernanza de Memoria y Controles de Privacidad](#uc-09-gobernanza-de-memoria-y-controles-de-privacidad)  
  - [UC-10: Copiloto de Runbook en Guardias](#uc-10-copiloto-de-runbook-en-guardias)  
- [Hoja de Ruta MVP (Primeras 4–6 Semanas)](#hoja-de-ruta-mvp-primeras-4-6-semanas)  
- [Principios de Diseño Clave Vinculados a Casos de Uso](#principios-de-diseño-clave-vinculados-a-casos-de-uso)  
- [Preguntas Abiertas](#preguntas-abiertas)  

---

## Introducción

### Visión General
DoveNest desarrolla **agentes de planificación cooperativa**: personas de IA que combinan las fortalezas de los **compañeros de conversación de IA** y de los **agentes de IA** en un todo integrado.

### Motivación
La inteligencia artificial se está volviendo cada vez más poderosa y está asumiendo roles centrales en el trabajo, la investigación, la educación y la vida cotidiana. La base de esto son **modelos** cada vez más capaces, sobre los que se construyen las **personas de IA**. Estas personas de IA determinan en gran medida cómo la gente puede hacer uso práctico de la IA.

### El Problema con las Personas de IA Existentes
Hoy en día, las personas de IA se dividen generalmente en dos categorías:

1. **Compañeros de conversación** – fuertes en el diálogo, la transferencia de conocimiento, la comprensión de temas complejos y la ideación creativa.  
2. **Agentes** – especializados en ejecutar tareas definidas con precisión a través de interfaces y herramientas.  

Los humanos, sin embargo, se mueven naturalmente entre la **planificación** y la **acción**. La separación actual de categorías causa fricción.

### La Solución DoveNest
Dado que ambas categorías suelen basarse en los mismos modelos subyacentes, no hay una razón técnica por la cual una sola persona de IA no pueda sobresalir en **ambas**. DoveNest proporciona exactamente esto: un **agente de planificación cooperativa** que unifica **comprensión, planificación y acción** en una interacción consistente.

### Características Clave de DoveNest
- **Memorias persistentes** para un compañero de conversación efectivo.  
- **Competencia en interfaces** para un agente efectivo.  
- **Interacción no lineal**: tareas largas se ejecutan en segundo plano mientras la conversación continúa.  
- **Memorias opcionales y contextuales**: activadas solo cuando son necesarias.  

---

## Arquitectura de DoveNest

### Las Tres Capas
1. **Interfaz de Usuario** – la capa visible donde los humanos interactúan con la IA.  
2. **Plataforma de Persona de IA** – la capa intermedia que da forma a la personalidad y capacidades.  
3. **Infraestructura de Modelos de IA** – proporciona la inteligencia y cómputo subyacente.

### El Papel de la Plataforma de Personas de IA
Determina lo que la IA realmente puede hacer, organizando contexto, memorias, interfaces e interacción. DoveNest une compañero de conversación y agente en uno solo.

### Compañeros de Conversación de IA
Diseñados para generar respuestas humanas y útiles, apoyados por memorias persistentes y notas de memoria.

### Agentes de IA
Ejecutan tareas a través de interfaces, pero pueden bloquear la conversación o ser ineficientes si no están bien integrados.

### Agentes de Planificación Cooperativa
Integran planificación y acción sin fricción, gestionan memorias, información y contexto, y permiten procesos asíncronos.

---

## Casos de Uso

### UC-01: Programador en Pareja Continuo con GitHub (sin cambio de modo)
**Problema:** Hoy es necesario cambiar de modo para que la IA haga cambios de código, interrumpiendo el flujo.  
**Visión:** Mantener la conversación mientras la IA propone, aplica y prueba cambios en GitHub, con CI ejecutándose en segundo plano.  
**MVP:** Un repositorio, una rama, un PR, pipeline con lint, fmt, tests.  
**KPIs:** Tiempo <1h para pasar CI; ≥80% PRs sin correcciones manuales.  

---

### UC-02: Refactorización Segura a Escala de Repositorio (Lotes + Guardarraíles)
**Problema:** Los refactors grandes son tediosos y propensos a errores.  
**Visión:** Planificación de refactor, cambios en lotes pequeños, tests asegurando estabilidad.  
**MVP:** Cambio de nombre + actualización de llamadas, docs actualizadas, tests verdes.  
**KPIs:** Tiempo medio de refactor, tasa de regresiones.  

---

### UC-03: Sincronización Multilingüe de Documentación y Libro Blanco
**Problema:** Las traducciones manuales se desincronizan.  
**Visión:** Una fuente autoritativa, IA mantiene sincronía multilingüe con glosarios.  
**MVP:** Sincronización DE↔EN para una página.  
**KPIs:** Latencia <24h, distancia de edición mínima.  

---

### UC-04: De la Investigación al PR (Lectura Profunda → Implementación)
**Problema:** Convertir especificaciones en código es lento.  
**Visión:** IA lee documento, extrae criterios, genera código mínimo y tests.  
**MVP:** De una especificación breve a un módulo con unit tests aprobados.  
**KPIs:** Cobertura de criterios, ciclos de revisión.  

---

### UC-05: Asistente de Triage y Reproducción de Errores
**Problema:** Faltan pasos de reproducción en issues.  
**Visión:** IA agrupa issues similares, genera casos mínimos de error, propone fix.  
**MVP:** Un caso fallido → test reproducible + PR con fix sugerido.  
**KPIs:** % issues reproducidos en <1h, tiempo a primer fix.  

---

### UC-06: Portal de Conocimiento sobre Código y Decisiones
**Problema:** Se pierde el conocimiento tácito sobre decisiones.  
**Visión:** Q&A natural sobre base de código + ADRs + PRs, con citas.  
**MVP:** Responder “¿Por qué usamos LanceDB en lugar de SQLite?” con citas.  
**KPIs:** Precisión de respuestas, tiempo de onboarding reducido.  

---

### UC-07: Automatización de Reunión a Backlog
**Problema:** Las decisiones de reuniones no llegan a convertirse en issues.  
**Visión:** IA extrae acciones de transcripciones, genera issues, vincula dependencias.  
**MVP:** De un transcript → 5–10 issues en GitHub.  
**KPIs:** % de acciones capturadas, satisfacción de stakeholders.  

---

### UC-08: Inicio de Pipeline de Datos/Embeddings
**Problema:** Configurar pipelines de embeddings es repetitivo.  
**Visión:** IA scaffold de microservicio (Rust + LanceDB), con métricas y CI/CD.  
**MVP:** Servicio local ejecutable con tests de humo.  
**KPIs:** Tiempo al primer query, latencia p95.  

---

### UC-09: Gobernanza de Memoria y Controles de Privacidad
**Problema:** Las memorias persistentes son útiles pero riesgosas.  
**Visión:** Controles granulares: opt-in, auditoría, retención, explicaciones.  
**MVP:** Aprobar/denegar memorias por conversación, con logs exportables.  
**KPIs:** Confianza de usuario, incidentes de privacidad = 0.  

---

### UC-10: Copiloto de Runbook en Guardias
**Problema:** Los incidentes requieren respuesta rápida.  
**Visión:** IA vincula alertas a runbooks, ejecuta diagnósticos, propone fixes, genera postmortems.  
**MVP:** Simulación de incidente → diagnósticos + borrador de postmortem.  
**KPIs:** Reducción de MTTR, precisión de sugerencias.  

---

## Hoja de Ruta MVP (Primeras 4–6 Semanas)
- **Semana 1–2:** Fundamentos UC-01 (repo mínimo + CI).  
- **Semana 3–4:** Sincronización DE↔EN + Portal de Conocimiento.  
- **Semana 5–6:** Piloto de Reproducción de Errores.  

---

## Principios de Diseño Clave Vinculados a Casos de Uso
- No hay cambio de modo duro: la conversación es primaria.  
- Humano en el bucle: siempre aprobación explícita.  
- Proveniencia en todo: links a PRs, tests, ADRs, logs.  
- Degradación elegante: planear aunque falte una herramienta.  
- Barandillas de seguridad: permisos mínimos, sandboxes, acciones firmadas.  

---

## Preguntas Abiertas
- ¿Modelo unificado de eventos para actualizaciones asíncronas?  
- ¿Interfaz estándar de tests entre lenguajes?  
- ¿Esquema de memoria portable entre proyectos y personas?  

