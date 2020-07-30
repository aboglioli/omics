<< ESPAÑOL >>

SOBRE PROYECTO

HERRAMIENTAS NECESARIAS
- NodeJs
- Angular Cli
- Docker
- Make (comandos)
-

CONFIGURACIÓN DOCKER
- Para windows:
  - Descargar el installador Docker de escritorio (versión estable) de: https://www.docker.com/get-started
  - Habilitar hyperV (verificar que este habilitado container y hyperV en Caractrerística de Windows)
  - Verificar que no se inicie siempre con Windows al iniciar (reiniciar la computadora una vez instalado)
  - Para verificar que funciona
    * Ir al administrador Hyper-V
    * Ver que DockerDesktop este ejecutandose (sino doble click y ejecutarlo)
    * En caso de tener el problema de que "Hyper-V no se esta ejectuando"
      - Abrir un CMD (o Powershell) con Permisos de Administrador
      - Para comprobar que Hyper-V este funcionando ejecutar: bcdedit  (ver si hypervisorlaunchtype este en auto)
      - Para cambiarlo ejecutar lo siguiente: bcdedit /set hypervisorlaunchtype auto
      - Reiniciar la máquina

- Para generar la imagen de frontend:
  * docker-compose up web
- Para generar la imagen de backend:
  * docker-compose up server

LEVANTAR BACKEND

-

LEVANTAR FRONTEND


CONSTRUIR PROYECTO


<< INGLÉS >>
