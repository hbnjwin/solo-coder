package com.linkyoyo.tobacco.entity;

import com.fasterxml.jackson.annotation.JsonFormat;
import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;
import org.codehaus.jackson.annotate.JsonIgnore;
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
@Table(name = "standard_system")
public class StandardSystem implements Serializable {
    @Id
    @Column(name="id")
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id; //id
    
    @Column(name = "system_code")
    private String systemCode; //体系code
    
    @Column(name = "system_name")
    private String systemName; //体系名称
    
    @Column(name = "system_level")
    private Integer systemLevel; //级别
    
    @Column(name = "parent_system")
    private Integer parentSystem; //上级体系
    
    @Column(name = "parent_system_code")
    private String parentSystemCode; //上级体系code
    
    @Column(name = "system_desc")
    private String systemDesc; //体系描述
    
    @Column(name = "remark")
    private String remark; //备注
    
    @Column(name = "parent_id")
    private Integer parentId; //父id

    @Column(name = "exclude_dept_id")
    @JsonIgnore
    private String excludeDeptId; //排除部门id

}
