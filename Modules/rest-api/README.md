# {{ project-name }}

Simple REST module that manages users in memory and exposes the same CRUD use cases through `ClientHub`.

## Endpoints

- `GET /{{ project-name }}/v1/users`
- `GET /{{ project-name }}/v1/users/{id}`
- `POST /{{ project-name }}/v1/users`
- `PATCH /{{ project-name }}/v1/users/{id}`
- `DELETE /{{ project-name }}/v1/users/{id}`

The storage is process-local and resets when the module restarts.
