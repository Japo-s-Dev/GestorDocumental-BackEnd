-- Esquema base de app

CREATE TABLE
    TIPO_DATO (
        tipoDatoId SERIAL PRIMARY KEY,
        nombre VARCHAR(50) NOT NULL
    );

CREATE TABLE
    PROYECTO (
        proyectoId SERIAL PRIMARY KEY,
        nombre VARCHAR(50) NOT NULL,
        plantilla VARCHAR(50) NOT NULL
    );

CREATE TABLE
    EXPEDIENTE (
        expedienteId SERIAL PRIMARY KEY,
        creacionFecha DATE NOT NULL,
        modificacionFecha DATE,
        usuarioCreador INT NOT NULL,
        usuarioMod INT,
        tag VARCHAR(50)
    );

CREATE TABLE 
    SEPARADOR (
        separadorId Serial PRIMARY KEY,
        nombre VARCHAR(50) NOT NULL,
        padreId INT NOT NULL,
        expedienteId INT NOT NULL,
        FOREIGN KEY (expedienteId) REFERENCES EXPEDIENTE(expedienteId) ON DELETE CASCADE,
        FOREIGN KEY (padreId) REFERENCES SEPARADOR(separadorId) ON DELETE CASCADE
    );

CREATE TABLE
    DOCUMENTO (
        documentoId SERIAL PRIMARY KEY,
        padreId INT NOT NULL,
        nombre VARCHAR(50) NOT NULL,
        tipoDocumento VARCHAR(50) NOT NULL,
        creacionFecha DATE NOT NULL,
        modificacionFecha DATE,
        usuarioCreador INT NOT NULL,
        usuarioMod INT,
        url VARCHAR(50) NOT NULL,
        FOREIGN KEY (padreId) REFERENCES SEPARADOR(separadorId) ON DELETE CASCADE
    );

CREATE TABLE
    INDICE (
        correlativo SERIAL NOT NULL,
        proyectoId INT NOT NULL,
        tipoDato INT NOT NULL,
        requerido VARCHAR(50),
        PRIMARY KEY (correlativo, proyectoId),
        FOREIGN KEY (proyectoId) REFERENCES PROYECTO(proyectoId) ON DELETE CASCADE,
        FOREIGN KEY (tipoDato) REFERENCES TIPO_DATO(tipoDatoId) ON DELETE CASCADE
    );

CREATE TABLE
    VALOR (
        correlativo INT NOT NULL,
        proyectoId INT NOT NULL,
        expedienteId INT NOT NULL,
        creacionFecha DATE NOT NULL,
        modificacionFecha DATE,
        usuarioMod INT,
        valor VARCHAR(50) NOT NULL,
        FOREIGN KEY (correlativo, proyectoId) REFERENCES INDICE(correlativo, proyectoId) ON DELETE CASCADE,
        FOREIGN KEY (proyectoId) REFERENCES PROYECTO(proyectoId) ON DELETE CASCADE,
        FOREIGN KEY (expedienteId) REFERENCES EXPEDIENTE(expedienteId) ON DELETE CASCADE
    );

CREATE TABLE
    "user" (
        id BIGSERIAL PRIMARY KEY,
        username VARCHAR(50) UNIQUE NOT NULL,
        email VARCHAR(50) UNIQUE NOT NULL,        
        pwd VARCHAR(256),
        pwd_salt uuid NOT NULL DEFAULT gen_random_uuid(),
        token_salt uuid NOT NULL DEFAULT gen_random_uuid()
    );

CREATE TABLE
    ROL (
        rolId SERIAL PRIMARY KEY,
        nombre VARCHAR(50) UNIQUE NOT NULL,
        descripcion VARCHAR(50)
    );

CREATE TABLE
    PERMISO (
        permisoId SERIAL PRIMARY KEY,
        nombre VARCHAR(50) UNIQUE NOT NULL,
        descripcion VARCHAR(50)
    );

CREATE TABLE
    ROL_ASIGNADO (
        userId INT NOT NULL,
        rolId INT NOT NULL,
        PRIMARY KEY (userId, rolId),
        FOREIGN KEY (userId) REFERENCES "user" (id) ON DELETE CASCADE,
        FOREIGN KEY (rolId) REFERENCES ROL (rolId) ON DELETE CASCADE
    );

CREATE TABLE
    PERMISO_ASOCIADO (
        permisoId INT NOT NULL,
        rolId INT NOT NULL,
        PRIMARY KEY (permisoId, rolId),
        FOREIGN KEY (permisoId) REFERENCES PERMISO (permisoId) ON DELETE CASCADE,
        FOREIGN KEY (rolId) REFERENCES ROL (rolId) ON DELETE CASCADE
    );

CREATE TABLE
    LOG_SESION (idLog SERIAL PRIMARY KEY);

CREATE TABLE
    DETALLE_LOG (
        idLog INT NOT NULL,
        userId INT NOT NULL,
        fecha DATE NOT NULL,
        hora TIME NOT NULL,
        accion VARCHAR(50),
        token VARCHAR(50),
        PRIMARY KEY (idLog, userId),
        FOREIGN KEY (idLog) REFERENCES LOG_SESION (idLog) ON DELETE CASCADE,
        FOREIGN KEY (userId) REFERENCES "user"(id) ON DELETE CASCADE
    );
