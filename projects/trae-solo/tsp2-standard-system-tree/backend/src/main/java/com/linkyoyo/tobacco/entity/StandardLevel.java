package com.linkyoyo.tobacco.entity;

import com.fasterxml.jackson.annotation.JsonFormat;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.hibernate.annotations.DynamicInsert;
import org.hibernate.annotations.DynamicUpdate;
import javax.persistence.*;
import java.io.Serializable;
import java.util.Date;

@Entity
@Data
@AllArgsConstructor
@NoArgsConstructor
@Builder
@DynamicInsert
@DynamicUpdate
@Table(name = "standard_level")
public class StandardLevel implements Serializable {
    @Id
    @Column(name="id")
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id; //id
    
    @Column(name = "standard_level_name")
    private String standardLevelName; //标准级别
    
    @Column(name = "standard_level_desc")
    private String standardLevelDesc; //描述
    
    @Column(name = "code")
    private String code; //级别id
    


}
