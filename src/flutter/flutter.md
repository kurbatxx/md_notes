# Flutter

build runner
```
flutter pub run build_runner build --delete-conflicting-outputs
```

### dep for freezed
```
dependencies:
    freezed_anotation:

dev_dependencies:
    freezed:
    build_runner:
    json_serializable:
```

### dep riverpod_lint
pubspec.yaml

```
dev_dependencies:
  custom_lint:
  riverpod_lint:
```

analysis_options.yaml
```
analyzer:
  plugins:
    - custom_lint
```

cli
```
dart run custom_lint
```