package com.pechariwi.gargoyle_lrd.config;

import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.jdbc.core.JdbcTemplate;
import org.springframework.jdbc.datasource.DriverManagerDataSource;

import javax.sql.DataSource;

@Configuration
public class ApplicationConfig {

    @Bean
    public DataSource dataSource(){
        DriverManagerDataSource dataSource = new DriverManagerDataSource();
        dataSource.setDriverClassName("com.mysql.cj.jdbc.Driver");

        // nombre de la base de datos
        String dbName = "";
        dataSource.setUrl("jdbc:mysql://localhost:3306/" + dbName);

        // nombre del usuario
        String dbUser = "spring_backend";
        dataSource.setUsername(dbUser);

        // contraseña
        String dbPass = "";
        dataSource.setPassword(dbPass);

        return dataSource;
    }

    @Bean
    public JdbcTemplate jdbcTemplate(DataSource dataSource){
        return new JdbcTemplate(dataSource);
    }
}
