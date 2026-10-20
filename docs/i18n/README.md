# INFO Desktop — localization

The localization model follows the same product rule used in Taxo: the application UI and public README are maintained in the same seven languages.

- Українська / `uk`
- English / `en`
- Deutsch / `de`
- Español / `es`
- Français / `fr`
- 한국어 / `ko`
- 日本語 / `ja`

## Rules

- User-facing UI text belongs in `src/locales/*.json`, not hard-coded in components.
- All locale files keep the same functional key set.
- README translations describe the same current release/status, not old prototype stages.
- UI language is independent of server instance language, channel language and content language.

## Public README translations

[Українська](../../README.md) · [English](README.en.md) · [Deutsch](README.de.md) · [Español](README.es.md) · [Français](README.fr.md) · [한국어](README.ko.md) · [日本語](README.ja.md)

The root [README](../../README.md) is Ukrainian and is the canonical product overview.
