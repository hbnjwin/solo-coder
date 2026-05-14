package com.linkyoyo.tobacco.entity;

import lombok.AllArgsConstructor;
import lombok.Builder;
import lombok.Data;
import lombok.NoArgsConstructor;

import javax.persistence.*;
import java.io.Serializable;
import java.util.Date;

/**
 * 标准规范性引用文件明细表
 */
@Data
@Entity
@Builder
@NoArgsConstructor
@AllArgsConstructor
@Table(name = "standard_normative_reference")
public class StandardNormativeReference implements Serializable {

    private static final long serialVersionUID = 1L;

    @Id
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id;

    /**
     * 标准文档ID
     */
    @Column(name = "standard_doc_id")
    private Integer standardDocId;

    /**
     * 标准号
     */
    @Column(name = "standard_no")
    private String standardNo;

    /**
     * 标准名称
     */
    @Column(name = "standard_name")
    private String standardName;

    /**
     * 原始引用文本
     */
    @Column(name = "original_text")
    private String originalText;

    /**
     * 创建时间
     */
    @Column(name = "create_date")
    private Date createDate;

    /**
     * 创建人
     */
    @Column(name = "creater")
    private String creater;

    /**
     * 创建人ID
     */
    @Column(name = "creater_id")
    private Integer createrId;

    /**
     * 删除标志
     */
    @Column(name = "del_flag")
    private Boolean delFlag;
}
