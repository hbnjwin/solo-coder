package com.linkyoyo.tobacco.entity;

import com.fasterxml.jackson.annotation.JsonFormat;
import com.fasterxml.jackson.annotation.JsonIgnoreProperties;
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
@Table(name = "standard_doc")
@JsonIgnoreProperties({"hibernateLazyInitializer", "handler"})
public class StandardDoc implements Serializable {
    @Id
    @Column(name="id")
    @GeneratedValue(strategy = GenerationType.IDENTITY)
    private Integer id; //id

    @Column(name = "standard_name")
    private String standardName; //标准名称

    @Column(name = "system_id")
    private Integer systemId; //标准体系id

    @Column(name = "system_code")
    private String systemCode; //标准体系code

    @Column(name = "system_name")
    private String systemName; //标准体系名称

    @Column(name = "standard_no")
    private String standardNo; //标准号

    @Column(name = "publish_date")
    @JsonFormat(pattern="yyyy-MM-dd HH:mm:ss",timezone = "GMT+8")
    private Date publishDate; //发布时间

    @Column(name = "status")
    private Integer status; //标准状态

    @Column(name = "status_label")
    private String statusLabel; //标准状态 未发布/现行有效/已经废止

    @Column(name = "standard_level_name")
    private String standardLevelName; //标准级别

    @Column(name = "standard_deparment_id")
    private Integer standardDeparmentId; //归口分标委

    @Column(name = "standard_deparment_name")
    private String standardDeparmentName; //归属分标委

    @Column(name = "group_id")
    private Integer groupId; //上传组id

    @Column(name = "group_name")
    private String groupName; //上传组

    @Column(name = "create_date")
    @JsonFormat(pattern="yyyy-MM-dd HH:mm:ss",timezone = "GMT+8")
    private Date createDate; //创建时间

    @Column(name = "creater")
    private String creater; //创建人

    @Column(name = "creater_id")
    private Integer createrId; //创建人id

    @Column(name = "remark")
    private String remark; //备注

    @Column(name = "file_url")
    private String fileUrl; //上传文件

    @Column(name = "upload_status")
    private String uploadStatus; //上传状态

    @Column(name = "code")
    private String code; //标准id

    @Column(name = "standard_deparment_code")
    private String standardDeparmentCode; //归口分标委

    @Column(name = "dept_id")
    private Integer deptId; //上传组织id

    @Column(name = "dept_name")
    private String deptName; //上传组织

    @Column(name = "operator_code")
    private String operatorCode; //用户id

    @Column(name = "mobile_phone")
    private String mobilePhone; //手机号

    @Column(name = "publish_unit")
    private String publishUnit; //发布单位

    @Column(name = "del_flag")
    private Boolean delFlag; //删除标记

    @Column(name = "standard_status")
    private String standardStatus; //标准状态

    @Column(name = "implement_date")
    private String implementDate; //实施日期

    @Column(name = "standard_en_name")
    private String standardEnName; //标准英文名

    @Column(name = "draft_unit", columnDefinition = "TEXT")
    private String draftUnit; //起草单位

    @Column(name = "drafter")
    private String drafter; //起草人

    @Column(name = "propose_unit", columnDefinition = "TEXT")
    private String proposeUnit; //提出单位

    @Column(name = "scope", columnDefinition = "TEXT")
    private String scope; //范围

    @Column(name = "introduction", columnDefinition = "TEXT")
    private String introduction; //引言

    @Column(name = "reference_docs", columnDefinition = "TEXT")
    private String referenceDocs; //参考文献

    @Column(name = "normative_references", columnDefinition = "TEXT")
    private String normativeReferences; //规范性引用文件

    @Column(name = "terms_and_definitions", columnDefinition = "TEXT")
    private String termsAndDefinitions; //术语和定义

    @Column(name = "preface", columnDefinition = "TEXT")
    private String preface; //前言

    @Column(name = "ai_analysis_result", columnDefinition = "TEXT")
    private String aiAnalysisResult; //AI分析结果

    @Column(name = "md_file_path")
    private String mdFilePath; //Markdown文件路径

    @Column(name = "simple_md_file_path")
    private String simpleMdFilePath; //简化版Markdown文件路径

    @Column(name = "is_graph")
    private Boolean isGraph = false; //是否图谱

    @Column(name = "is_corrected_normative_reference")
    private Boolean isCorrectedNormativeReference = false; //是否修正规范引用

    @Column(name = "xml_content", columnDefinition = "TEXT")
    private String xmlContent; // XML结构化内容


}
